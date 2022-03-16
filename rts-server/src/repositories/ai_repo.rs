use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::ai::NewAi;

pub struct AiRepository;

impl AiRepository {
    pub async fn insert<'a>( // @TODO remove this lifetime
        pool: &PgPool,
        ai: NewAi<'a>,
    ) -> Result<(), WebServerException> {
        sqlx::query("INSERT INTO ais (owner, code) VALUES ($1, $2)")
            .bind(ai.owner)
            .bind(ai.code)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|_| {
                WebServerException::SqlException(format!("Failed to insert ai {:?}", ai))
            })
    }


}
