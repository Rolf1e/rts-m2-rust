
use diesel::prelude::*;
use actix_web::{HttpResponse, get};
use actix_web::{Responder, web};

use crate::AppState;

use crate::models::*;
use crate::schema::*;


#[get("/leaderboard")]
pub async fn leaderboard(state: web::Data<AppState<'_>>) -> impl Responder {
    // TODO
    let conn = state.pool.get().expect("Could not connect to the database");
    let users = users::table
        .load::<User>(&conn)
        .expect("Error loading users");
    dbg!(users);
    HttpResponse::Ok().body("Leaderboard")
}
