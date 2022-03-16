use sqlx::PgPool;

use crate::exceptions::WebServerException;
use crate::models::user::{NewUser, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Vec<User>, WebServerException> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, password, email FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            WebServerException::SqlException(format!(
                "Failed to fetch users with username {}",
                username
            ))
        })
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Vec<User>, WebServerException> {
        sqlx::query_as::<_, User>("SELECT id, username, password, email FROM users WHERE id = $1")
            .bind(id)
            .fetch_all(pool)
            .await
            .map_err(|_| {
                WebServerException::SqlException(format!("Failed to fetch users with id {}", id))
            })
    }

    pub async fn insert_user<'a>(
        pool: &PgPool,
        user: NewUser<'a>,
    ) -> Result<(), WebServerException> {
        sqlx::query("INSERT INTO users (username, password, email) VALUES ($1, $2, $3)")
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|_| {
                WebServerException::SqlException(format!("Failed to insert user {:?}", user))
            })
    }
}
