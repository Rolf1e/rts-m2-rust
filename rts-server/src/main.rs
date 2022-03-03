#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use actix_files as fs;
use actix_web::web::Data;
use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use argon2::{self, Config};
use cookie::time::Duration;
use cookie::Cookie;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use rts_core::components::game::Game;
use rts_core::entity::game_actions::Action;
use rts_core::entity::player::Player;
use rts_core::entity::unit::UnitType;

use self::models::*;
use self::schema::*;

const AUTH_COOKIE_NAME: &str = "_token";

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState<'a> {
    pool: PostgresPool,
    argon2_config: Config<'a>,
    tokens: Arc<RwLock<HashMap<String, i32>>>,
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
) -> Result<User, String> {
    let mut salt = vec![0u8; 64];
    OsRng.fill_bytes(&mut salt);

    let hashed_password = argon2::hash_encoded(password.as_bytes(), &salt, config).unwrap();
    let new_user = NewUser {
        username,
        password: &hashed_password,
        email,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
    {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum RegisterResult {
    Successful,
    Failed(String),
}

#[post("/register")]
async fn register(
    info: web::Json<RegisterInfo>,
    req: HttpRequest,
    state: web::Data<AppState<'_>>,
) -> impl Responder {
    let conn = state.pool.get().expect("Could not connect to the database");
    println!(
        "Register request from {} with password {} and mail {}",
        info.username, info.password, info.email
    );
    match create_user(
        &conn,
        &info.username,
        &info.password,
        &info.email,
        &state.argon2_config,
    ) {
        Ok(user) => {
            println!("User registered with id {}", user.id);
            web::Json(RegisterResult::Successful).respond_to(&req)
        }
        Err(message) => {
            println!("Registration failed: {}", &message);
            let mut response = web::Json(RegisterResult::Failed(message)).respond_to(&req);
            *response.status_mut() = actix_web::http::StatusCode::BAD_REQUEST;
            response
        }
    }
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
enum LoginResult {
    ValidLogin { username: String, user_id: i32 },
    InvalidLogin { message: String },
}

#[post("/login")]
async fn login(
    info: web::Json<LoginInfo>,
    req: HttpRequest,
    state: web::Data<AppState<'_>>,
) -> impl Responder {
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
        let mut response = web::Json(LoginResult::InvalidLogin {
            message: "Invalid username or password".to_string(),
        })
        .respond_to(&req);
        *response.status_mut() = actix_web::http::StatusCode::UNAUTHORIZED;
        response
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
            let mut response = web::Json(LoginResult::ValidLogin {
                username: matching_user.username.to_string(),
                user_id: matching_user.id,
            })
            .respond_to(&req);
            response
                .add_cookie(
                    &Cookie::build(AUTH_COOKIE_NAME, token)
                        .max_age(Duration::days(31))
                        .finish(),
                )
                .expect("Could not build a valid cookie");
            response
        } else {
            let mut response = web::Json(LoginResult::InvalidLogin {
                message: "Invalid username or password".to_string(),
            })
            .respond_to(&req);
            *response.status_mut() = actix_web::http::StatusCode::UNAUTHORIZED;
            response
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
enum LoginState {
    LoggedOut,
    LoggedIn { username: String, user_id: i32 },
}

#[get("/login_status")]
async fn login_status(req: HttpRequest, state: web::Data<AppState<'_>>) -> impl Responder {
    match get_current_user(&req, &state) {
        None => web::Json(LoginState::LoggedOut),
        Some(user) => web::Json(LoginState::LoggedIn {
            username: user.username,
            user_id: user.id,
        }),
    }
}

#[post("/logout")]
async fn logout() -> impl (Responder) {
    HttpResponse::Ok()
        .cookie(
            Cookie::build(AUTH_COOKIE_NAME, "")
                .max_age(Duration::ZERO)
                .finish(),
        )
        .body("Logged out")
}

fn get_current_user(req: &HttpRequest, state: &web::Data<AppState<'_>>) -> Option<Box<User>> {
    use self::schema::users::dsl::*;
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

#[derive(Serialize)]
#[serde(untagged)]
enum AiResult {
    Successful,
    Failed(String),
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
        None => {
            let mut response =
                web::Json(AiResult::Failed("You are not logged in.".to_string())).respond_to(&req);
            *response.status_mut() = actix_web::http::StatusCode::UNAUTHORIZED;
            return response;
        }
        Some(user) => user,
    };
    println!("Found an user matching the cookies");

    // Save the AI in the database
    println!("Ai submit request with ai {:?} for user {:?}", &info, &user);
    let code = match info.0 {
        AiInfo::Ai(c) => c,
        AiInfo::PastebinKey(key) => match fetch_ai_from_pastebin(&key).await {
            None => {
                let mut response = web::Json(AiResult::Failed(
                    "Could not fetch code from pastebin.".to_string(),
                ))
                .respond_to(&req);
                *response.status_mut() = actix_web::http::StatusCode::SERVICE_UNAVAILABLE;
                return response;
            }
            Some(c) => c,
        },
        AiInfo::Gist { username, hash } => match fetch_ai_from_gist(&username, &hash).await {
            None => {
                let mut response = web::Json(AiResult::Failed(
                    "Could not fetch code from gist.".to_string(),
                ))
                .respond_to(&req);
                *response.status_mut() = actix_web::http::StatusCode::SERVICE_UNAVAILABLE;
                return response;
            }
            Some(c) => c,
        },
    };

    let conn = state.pool.get().expect("Could not connect to the database");
    let new_ai = NewAi {
        owner: user.id,
        code: &code,
    };
    match diesel::insert_into(ais::table)
        .values(&new_ai)
        .get_result::<AI>(&conn)
    {
        Ok(ai) => {
            println!("AI created with id {}", ai.id);
            web::Json(AiResult::Successful).respond_to(&req)
        }
        Err(err) => {
            println!("Ai submit failed: {}", &err);
            let mut response = web::Json(AiResult::Failed(err.to_string())).respond_to(&req);
            *response.status_mut() = actix_web::http::StatusCode::BAD_REQUEST;
            response
        }
    }
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
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let listen_url = dotenv::var("LISTEN_URL").expect("LISTEN_URL must be set");
    println!("Listening on {}", &listen_url);

    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Could not build connection pool");

    let tokens = Arc::new(RwLock::new(HashMap::new()));

    HttpServer::new(move || {
        let app_state: Data<AppState> = Data::new(AppState {
            pool: pool.clone(),
            argon2_config: Config::default(),
            tokens: tokens.clone(),
        });

        let api_scope = web::scope("/api")
            // login route
            .service(login)
            // login status route
            .service(login_status)
            // logout route
            .service(logout)
            // register route
            .service(register)
            // ai submit route
            .service(submit_ai)
            // leaderboard route
            .service(leaderboard);

        // TODO not great, we should only use this for routes defined in the front, and send a 404 for the rest
        let index_fallback = fs::NamedFile::open("./rts-server/static/index.html")
            .expect("Could not load the fallback index file.");

        let static_service = fs::Files::new("/", "./rts-server/static/")
            .index_file("index.html")
            .default_handler(index_fallback);

        App::new()
            // bind the database
            .app_data(app_state)
            // enable logger
            .wrap(middleware::Logger::default())
            // add the api routes
            .service(api_scope)
            // default to files in ./rts-server/static
            .service(static_service)
    })
    .bind(listen_url)?
    .run()
    .await
}
