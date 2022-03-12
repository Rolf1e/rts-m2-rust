use crate::AppState;
use actix_web::{get, post, HttpResponse};
use actix_web::{web, Responder};
use diesel::prelude::*;

use crate::dto::input::NewMatchDto;
use crate::dto::output::MatchResult;
use crate::models::game::*;
use crate::schema::*;

#[post("/leaderboard")]
pub async fn insert_new_match(
    state: web::Data<AppState<'_>>,
    new_match_dto: web::Json<NewMatchDto>,
) -> impl Responder {
    use crate::models::game::MatchDo;
    let conn = state
        .pool
        .get()
        .expect("Failed to acquire connexion when retrieving leader board");

    let dto = new_match_dto.into_inner();
    let insert_result = diesel::insert_into(matchs::table)
        .values(prepare_dto_for_insert(dto))
        .get_result::<MatchDo>(&conn);

    match insert_result {
        Ok(_) => HttpResponse::Ok().body("Successfuly register new match !"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to register new match :("),
    }
}

fn prepare_dto_for_insert(dto: NewMatchDto) -> NewMatchDo {
    NewMatchDo {
        winner: dto.winner_id,
        looser: dto.looser_id,
        score_winner: dto.score_winner,
        score_looser: dto.score_looser,
    }
}

#[get("/leaderboard/{max}")]
pub async fn leaderboard(state: web::Data<AppState<'_>>, max: web::Path<i32>) -> impl Responder {
    let max: i64 = max.abs().into();
    let conn = state
        .pool
        .get()
        .expect("Failed to acquire connexion when retrieving leader board");

    let leader_board = matchs::table
        .limit(max)
        .load::<MatchDo>(&conn)
        .map(|ranks| transform_result(&ranks));

    match leader_board {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve leader board"),
    }
}

fn transform_result(matchs: &[MatchDo]) -> Vec<MatchResult> {
    matchs
        .iter()
        .map(|m| MatchResult {
            winner: m.winner.to_string(), // TODO replace with real names
            looser: m.looser.to_string(),
            score_winner: m.score_winner,
            score_looser: m.score_looser,
        })
        .collect()
}
