//! Simple echo websocket server.
//! Open `http://localhost:8080/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::env;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::http::Cookie;
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use argon2::{self, Config};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::Deserialize;
use serde::Deserialize;

use rts_core::components::game::Game;
use rts_core::entity::game_actions::Action;
use rts_core::entity::player::Player;
use rts_core::entity::unit::UnitType;

use self::models::*;
use self::schema::*;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

const AUTH_COOKIE_NAME: &str = "_token";

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState<'a> {
    pool: PostgresPool,
    argon2_config: Config<'a>,
    tokens: Arc<RwLock<HashMap<String, i32>>>,
}

/// do websocket handshake and start `MyWebSocket` actor
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", r);
    let res = ws::start(MyWebSocket::new(), &r, stream);
    println!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[derive(Deserialize)]
struct RegisterInfo {
    username: String,
    password: String,
    email: String,
}

pub fn create_user<'a>(
    conn: &PgConnection,
    username: &'a str,
    password: &'a str,
    email: &'a str,
    config: &Config,
) -> User {
    let mut salt = vec![0u8; 64];
    OsRng.fill_bytes(&mut salt);

    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, config).unwrap();
    let new_user = NewUser {
        username,
        password: &hashed_password,
        email,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating user")
}

#[post("/register")]
async fn register(info: web::Json<RegisterInfo>, state: web::Data<AppState<'_>>) -> impl Responder {
    let conn = state.pool.get().expect("Could not connect to the database");
    println!(
        "Register request from {} with password {} and mail {}",
        info.username, info.password, info.email
    );
    let user = create_user(
        &conn,
        &info.username,
        &info.password,
        &info.email,
        &state.argon2_config,
    );
    println!("User registered with id {}", user.id);
    HttpResponse::Ok().body("Registered user")
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, state: web::Data<AppState<'_>>) -> impl Responder {
    use self::schema::users::dsl::*;
    let conn = state.pool.get().expect("Could not connect to the database");
    let matching_users = users
        .filter(username.eq(info.username.clone()))
        .load::<User>(&conn)
        .expect("Error loading users");
    println!(
        "Login request from {} with password {}",
        info.username, info.password
    );
    if matching_users.is_empty() {
        HttpResponse::BadRequest().body("Invalid username or password")
    } else if matching_users.len() > 1 {
        panic!("Found multiple matching usernames!")
    } else {
        let matching_user = &matching_users[0];
        println!("Found matching user {:?}", matching_user);
        let is_valid =
            argon2::verify_encoded(&matching_user.password, info.password.as_bytes()).unwrap();
        if is_valid {
            let mut raw_token = vec![0u8; 64];
            OsRng.fill_bytes(&mut raw_token);
            let token = base64::encode(&raw_token);
            state
                .tokens
                .write()
                .expect("Couldn't access the token storage")
                .insert(token.clone(), matching_user.id);
            println!("Saved token {} for user id {}.", &token, matching_user.id);
            HttpResponse::Ok()
                .cookie(
                    Cookie::build(AUTH_COOKIE_NAME, token)
                        .max_age(time::Duration::days(31))
                        .finish(),
                )
                .body("Logged in")
        } else {
            HttpResponse::BadRequest().body("Invalid username or password")
        }
    }
}

#[post("/logout")]
async fn logout() -> impl (Responder) {
    HttpResponse::Ok()
        .cookie(
            Cookie::build(AUTH_COOKIE_NAME, "")
                .max_age(time::Duration::zero())
                .finish(),
        )
        .body("Logged out")
}

fn get_current_user(req: &HttpRequest, state: &web::Data<AppState<'_>>) -> Option<Box<User>> {
    use self::schema::users::dsl::*;
    use actix_web::HttpMessage;
    println!("Fetching the user from the cookies");
    // Read the authentication cookie
    let auth_token = match req.cookie(AUTH_COOKIE_NAME) {
        None => {
            println!("Could not find the cookie in the headers");
            return None;
        }
        Some(cookie) => cookie.value().to_string(),
    };
    println!("Trying to find the user id for token {}", &auth_token);
    // Try to find the token in the app state
    let user_id = match state
        .tokens
        .read()
        .expect("Couldn't access the token storage")
        .get(&auth_token)
    {
        None => {
            println!("Could not find the token in the storage");
            return None;
        }
        Some(uid) => *uid,
    };
    // Find the user in the database
    let conn = state.pool.get().expect("Could not connect to the database");
    let matching_users = users
        .filter(id.eq(user_id))
        .load::<User>(&conn)
        .expect("Error loading users");
    if matching_users.is_empty() {
        panic!("Token is valid but the user was not found in the database!")
    } else if matching_users.len() > 1 {
        panic!("Found multiple users with the given id!")
    } else {
        Some(Box::new(matching_users[0].clone()))
    }
}

#[derive(Deserialize, Debug)]
enum AiInfo {
    Ai(String),
    PastebinKey(String),
    Gist { username: String, hash: String },
}

async fn fetch_ai_from_pastebin(paste_key: &str) -> Option<String> {
    reqwest::get(&format!(
        "https://pastebin.com/raw/{pastebin_key}",
        pastebin_key = paste_key
    ))
    .await
    .ok()?
    .text()
    .await
    .ok()
}

async fn fetch_ai_from_gist(username: &str, hash: &str) -> Option<String> {
    reqwest::get(&format!(
        "https://gist.githubusercontent.com/{username}/{hash}/raw",
        username = username,
        hash = hash
    ))
    .await
    .ok()?
    .text()
    .await
    .ok()
}

#[post("/submit_ai")]
async fn submit_ai(
    req: HttpRequest,
    state: web::Data<AppState<'_>>,
    info: web::Json<AiInfo>,
) -> impl (Responder) {
    // Authenticate the user
    let user = match get_current_user(&req, &state) {
        None => return HttpResponse::BadRequest().body("Not currently logged in"),
        Some(user) => user,
    };
    println!("Found an user matching the cookies");

    // Save the AI in the database
    println!("Ai submit request with ai {:?} for user {:?}", &info, &user);
    let code = match info.0 {
        AiInfo::Ai(c) => c,
        AiInfo::PastebinKey(key) => match fetch_ai_from_pastebin(&key).await {
            None => {
                return HttpResponse::ExpectationFailed()
                    .body("Could not fetch code from pastebin.")
            }
            Some(c) => c,
        },
        AiInfo::Gist { username, hash } => match fetch_ai_from_gist(&username, &hash).await {
            None => {
                return HttpResponse::ExpectationFailed().body("Could not fetch code from gist.")
            }
            Some(c) => c,
        },
    };

    let conn = state.pool.get().expect("Could not connect to the database");
    let new_ai = NewAi {
        owner: user.id,
        code: &code,
    };
    let ai: AI = diesel::insert_into(ais::table)
        .values(&new_ai)
        .get_result(&conn)
        .expect("Error creating ai");
    println!("AI created with id {}", ai.id);

    HttpResponse::Ok().body("Submitted AI")
}

#[get("/leaderboard")]
async fn leaderboard(state: web::Data<AppState<'_>>) -> impl Responder {
    // TODO
    let conn = state.pool.get().expect("Could not connect to the database");
    let users = users::table
        .load::<User>(&conn)
        .expect("Error loading users");
    dbg!(users);
    HttpResponse::Ok().body("Leaderboard")
}

async fn start(game: &Game) {
    println!("Launch game");
    match game.start().await {
        Ok(_) => println!("GS"),
        Err(e) => println!("{}", e),
    }
}

async fn play(game: &Game, action: Action) {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let action_name = action.get_name();
    match game.play_with_async(0, action).await {
        Ok(_) => println!("Play {}", action_name),
        Err(e) => println!("{}", e),
    }

    game.console_display().unwrap();
}

async fn run_game() {
    let p1 = Player::new("Tigran".to_string());
    let p2 = Player::new("Emma".to_string());
    let game = Game::new(vec![p1, p2]);

    game.console_display().unwrap();

    let t1 = start(&game);

    let turn = play(&game, Action::BuyUnit(UnitType::Classic)).await;
    tokio::select! {
        _ = t1 => println!("Game started"),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the rts web server");
    //std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let listen_url = dotenv::var("LISTEN_URL").expect("LISTEN_URL must be set");

    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Could not build connection pool");

    let tokens = Arc::new(RwLock::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            // bind the database
            .data(AppState {
                pool: pool.clone(),
                argon2_config: Config::default(),
                tokens: tokens.clone(),
            })
            // enable logger
            .wrap(middleware::Logger::default())
            // login route
            .service(login)
            // logout route
            .service(logout)
            // register route
            .service(register)
            // ai submit route
            .service(submit_ai)
            // leaderboard route
            .service(leaderboard)
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // default to files in ./rts-server/static
            .service(fs::Files::new("/", "./rts-server/static/").index_file("index.html"))
    })
    .bind(listen_url)?
    .run()
    .await
}
