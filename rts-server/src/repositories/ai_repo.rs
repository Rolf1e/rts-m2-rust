use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::ai::NewAi;

pub struct AiRepository;

const TABLE_NAME: &str = "ais";

impl AiRepository {
    pub async fn insert(pool: &PgPool, ai: NewAi) -> Result<(), WebServerException> {
        let owner = ai.owner;
        sqlx::query(&format!(
            "INSERT INTO {} (owner, code) VALUES ($1, $2)",
            TABLE_NAME
        ))
        .bind(ai.owner)
        .bind(ai.code)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|_| WebServerException::Sql(format!("Failed to insert ai {:?}", owner)))
    }
}
