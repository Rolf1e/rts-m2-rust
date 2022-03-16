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

    HttpResponse::InternalServerError().body("todo !")
}

fn prepare_dto_for_insert(dto: NewMatchDto) -> NewMatchDo {
    NewMatchDo {
        game: dto.game,
        player: dto.player,
        score: dto.score,
    }
}

#[get("/leaderboard/{max}")]
pub async fn leaderboard(state: web::Data<AppState<'_>>, max: web::Path<i32>) -> impl Responder {
    let _max: i64 = max.abs().into();

    let leader_board = sqlx::query_as::<_, LeaderBoardRowDo>(
        "SELECT (SELECT username
                    FROM users
                    WHERE users.id = games.player) AS name,
                   SUM(games.score)                AS total_score
            FROM (SELECT player, MAX(SCORE) AS score
                  FROM matchs
                  GROUP BY 1, matchs.game) AS games
            GROUP BY 1
            ORDER BY total_score DESC",
    )
    .fetch_all(&state.pg_pool)
    .await
    .map(transform_to_dto);

    match leader_board {
        Ok(board) => {
            println!("{:?}", board);
            HttpResponse::Ok().json(board)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to retrieve leader board"),
    }
}

fn transform_to_dto(leader_board: Vec<LeaderBoardRowDo>) -> Vec<LeaderBoardDto> {
    println!("{:?}", leader_board);
    leader_board
        .iter()
        .map(|row| LeaderBoardDto {
            username: row.name.clone(),
            score: row.total_score,
            wins: 0,
            looses: 0,
        })
        .collect()
}
