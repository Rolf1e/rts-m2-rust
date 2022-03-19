use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::game::{LeaderBoardRowDo, NewMatchDo};

use super::user_repo;

pub struct MatchRepository;

const TABLE_NAME: &str = "matchs";

impl MatchRepository {
    pub async fn insert(pool: &PgPool, new_match: NewMatchDo) -> Result<(), WebServerException> {
        sqlx::query(&format!(
            "INSERT INTO {} (player, game, score) VALUES ($1, $2, $3)",
            TABLE_NAME
        ))
        .bind(new_match.player)
        .bind(new_match.game)
        .bind(new_match.score)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|_| WebServerException::Sql(format!("Failed to insert match {:?}", new_match)))
    }

    pub async fn fetch_leader_board(
        pool: &PgPool,
        max: i64,
    ) -> Result<Vec<LeaderBoardRowDo>, WebServerException> {
        let select_names_query = format!(
            "SELECT username FROM {} WHERE users.id = games.player",
            user_repo::TABLE_NAME
        );
        let select_player_score_query = format!(
            "SELECT player, MAX(SCORE) AS score FROM {} GROUP BY 1, matchs.game",
            TABLE_NAME
        );
        let select_wins_per_player = format!(
            "SELECT player, COUNT(winner) as total_wins FROM {} WHERE winner GROUP BY player",
            TABLE_NAME
        );
        let select_looses_per_player = format!(
            "SELECT player, COUNT(winner) as total_looses FROM {} WHERE NOT winner GROUP BY player",
            TABLE_NAME
        );

        let select_name_score_query = format!(
            "SELECT
                ({}) AS name,
                SUM(games.score) AS total_score,
                wins.total_wins AS wins,
                looses.total_looses AS looses
        FROM ({}) AS games
        JOIN ({}) AS wins ON games.player = wins.player
        JOIN ({}) AS looses ON games.player = looses.player
        GROUP BY 1, total_wins, total_looses
        ORDER BY total_score DESC
        limit $1",
            select_names_query,
            select_player_score_query,
            select_wins_per_player,
            select_looses_per_player
        );

        sqlx::query_as::<_, LeaderBoardRowDo>(&select_name_score_query)
            .bind(max)
            .fetch_all(pool)
            .await
            .map_err(|_| WebServerException::Sql(String::from("Failed to fetch leader board")))
    }
}
