#[macro_use]
extern crate diesel;

pub mod controllers;
pub mod dto;
pub mod models;
pub mod schema;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::controllers::leader_board_controller::leaderboard;
use crate::controllers::ai_controller::submit_ai;
use crate::controllers::user_controller::{login, login_status, logout, register};

use actix_files as fs;
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};
use argon2::{self, Config};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;


pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct AppState<'a> {
    pool: PostgresPool,
    argon2_config: Config<'a>,
    tokens: Arc<RwLock<HashMap<String, i32>>>,
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
