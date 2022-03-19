use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::user::{NewUser, User};

pub struct UserRepository;

pub const TABLE_NAME: &str = "users";

impl UserRepository {
    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Vec<User>, WebServerException> {
        sqlx::query_as::<_, User>(&format!(
            "SELECT id, username, password, email FROM {} WHERE username = $1",
            TABLE_NAME
        ))
        .bind(username)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            WebServerException::Sql(format!("Failed to fetch users with username {}", username))
        })
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Vec<User>, WebServerException> {
        sqlx::query_as::<_, User>(&format!(
            "SELECT id, username, password, email FROM {} WHERE id = $1",
            TABLE_NAME
        ))
        .bind(id)
        .fetch_all(pool)
        .await
        .map_err(|_| WebServerException::Sql(format!("Failed to fetch users with id {}", id)))
    }

    pub async fn insert(pool: &PgPool, user: NewUser) -> Result<(), WebServerException> {
        let username = user.username.clone();
        sqlx::query(&format!(
            "INSERT INTO {} (username, password, email) VALUES ($1, $2, $3)",
            TABLE_NAME
        ))
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(|_| WebServerException::Sql(format!("Failed to insert user {:?}", username)))
    }
}
