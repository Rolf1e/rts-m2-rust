use crate::AppState;
use actix_web::{get, post, HttpResponse};
use actix_web::{web, Responder};
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::dto::input::NewMatchDto;
use crate::dto::output::LeaderBoardDto;
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
    let _max: i64 = max.abs().into();
    let conn = state
        .pool
        .get()
        .expect("Failed to acquire connexion when retrieving leader board");

    let query = diesel::sql_query(
        "SELECT username, wins, looses, score_wins - score_looses AS score
    FROM (SELECT winner AS player, COUNT(winner) AS wins, SUM(score_winner) AS score_wins FROM matchs GROUP BY winner) as winners
    JOIN (SELECT looser AS player, COUNT(looser) AS looses, SUM(score_looser) AS score_looses FROM matchs GROUP BY looser) as loosers
    ON winners.player = loosers.player
    JOIN users ON users.id = winners.player",
    );

    println!("{}", diesel::debug_query::<Pg,_>(&query).to_string());
    let leader_board = query.load::<LeaderBoardRowDo>(&conn).map(transform_to_dto);

    match leader_board {
        Ok(board) => {
            println!("{:?}", board);
            HttpResponse::Ok().json(board)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve leader board"),
    }
}

fn transform_to_dto(leader_board: Vec<LeaderBoardRowDo>) -> Vec<LeaderBoardDto> {
    leader_board
        .iter()
        .map(|row| LeaderBoardDto {
            username: row.username.clone(),
            score: row.score,
            wins: row.wins,
            looses: row.looses,
        })
        .collect()
}
