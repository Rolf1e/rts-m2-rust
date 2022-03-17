use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::game::{LeaderBoardRowDo, NewMatchDo};

pub struct MatchRepository;

impl MatchRepository {
    pub async fn insert(pool: &PgPool, new_match: NewMatchDo) -> Result<(), WebServerException> {
        sqlx::query("INSERT INTO matchs (player, game, score) VALUES ($1, $2, $3)")
            .bind(new_match.player)
            .bind(new_match.game)
            .bind(new_match.score)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|_| {
                WebServerException::SqlException(format!("Failed to insert match {:?}", new_match))
            })
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<LeaderBoardRowDo>, WebServerException> {
        let select_names_query = "SELECT username FROM users WHERE users.id = games.player";
        let select_player_score_query =
            "SELECT player, MAX(SCORE) AS score FROM matchs GROUP BY 1, matchs.game";
        let select_name_score_query = format!(
            "SELECT 
            ({}) AS name,
            SUM(games.score) AS total_score
        FROM ({}) AS games
        GROUP BY 1
        ORDER BY total_score DESC",
            select_names_query, select_player_score_query
        );

        sqlx::query_as::<_, LeaderBoardRowDo>(&select_name_score_query)
            .fetch_all(pool)
            .await
            .map_err(|_| {
                WebServerException::SqlException(String::from("Failed to fetch leader board"))
            })
    }
}
