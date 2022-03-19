use crate::repositories::leader_repo::MatchRepository;
use crate::AppState;
use actix_web::{get, post, HttpResponse};
use actix_web::{web, Responder};

use crate::dto::input::NewMatchDto;
use crate::dto::output::LeaderBoardDto;
use crate::models::game::*;

#[post("/leaderboard")]
pub async fn insert_new_match(
    state: web::Data<AppState<'_>>,
    new_match_dto: web::Json<NewMatchDto>,
) -> impl Responder {
    let dto = new_match_dto.into_inner();
    match MatchRepository::insert(&state.pg_pool, prepare_dto_for_insert(dto)).await {
        Ok(_) => HttpResponse::Ok().body("Successfuly insert match"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

fn prepare_dto_for_insert(dto: NewMatchDto) -> NewMatchDo {
    NewMatchDo {
        game: dto.game,
        player: dto.player,
        score: dto.score,
        winner: dto.winner,
    }
}

#[get("/leaderboard/{max}")]
pub async fn leaderboard(state: web::Data<AppState<'_>>, max: web::Path<i32>) -> impl Responder {
    let max: i64 = max.abs().into();

    let leader_board = MatchRepository::fetch_leader_board(&state.pg_pool, max)
        .await
        .map(transform_to_dto);

    dbg!(&leader_board);

    match leader_board {
        Ok(board) => HttpResponse::Ok().json(board),
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("Failed to retrieve leader board")
        }
    }
}

fn transform_to_dto(leader_board: Vec<LeaderBoardRowDo>) -> Vec<LeaderBoardDto> {
    leader_board
        .iter()
        .map(|row| LeaderBoardDto {
            username: row.name.clone(),
            score: row.total_score,
            wins: row.wins,
            looses: row.looses,
        })
        .collect()
}
