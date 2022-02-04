//! Simple echo websocket server.
//! Open `http://localhost:8080/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{
    get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::Deserialize;

use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use self::models::User;
use self::schema::users;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
    pool: PostgresPool,
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

#[post("/register")]
async fn register(info: web::Json<RegisterInfo>) -> impl Responder {
    // TODO
    println!(
        "Register request from {} with password {} and mail {}",
        info.username, info.password, info.email
    );
    HttpResponse::Ok().body("Register")
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(info: web::Json<LoginInfo>) -> impl Responder {
    // TODO
    println!(
        "Login request from {} with password {}",
        info.username, info.password
    );
    HttpResponse::Ok().body("Login")
}

#[post("/logout")]
async fn logout() -> impl (Responder) {
    // TODO
    println!("Logout request");
    HttpResponse::Ok().body("Logout")
}

#[derive(Deserialize)]
struct AiInfo {
    ai: String,
}

#[post("/submit_ai")]
async fn submit_ai(info: web::Json<AiInfo>) -> impl (Responder) {
    // TODO
    println!("Ai submit request with ai {}", info.ai);
    HttpResponse::Ok().body("Submit AI")
}

#[get("/leaderboard")]
async fn leaderboard(state: web::Data<AppState>) -> impl Responder {
    // TODO
    let conn = state.pool.get().expect("Could not connect to the database");
    let users = users::table
        .load::<User>(&conn)
        .expect("Error loading users");
    dbg!(users);
    HttpResponse::Ok().body("Leaderboard")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let listen_url = dotenv::var("LISTEN_URL").expect("LISTEN_URL must be set");

    let pool = r2d2::Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Could not build connection pool");

    HttpServer::new(move || {
        App::new()
            // bind the database
            .data(AppState { pool: pool.clone() })
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
