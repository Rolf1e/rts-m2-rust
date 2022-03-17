use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use argon2::{self, Config};
use cookie::time::Duration;
use cookie::Cookie;
use rand::rngs::OsRng;
use rand::RngCore;
use sqlx::PgPool;

use crate::dto::input::{LoginInfo, RegisterInfo};
use crate::dto::output::{LoginResult, LoginState, RegisterResult};
use crate::exceptions::WebServerException;
use crate::models::user::*;
use crate::repositories::user_repo::UserRepository;
use crate::AppState;

const AUTH_COOKIE_NAME: &str = "_token";

#[post("/register")]
pub async fn register(
    info: web::Json<RegisterInfo>,
    state: web::Data<AppState<'_>>,
) -> impl Responder {
    let username = info.username.clone();
    match create_user(&state.pg_pool, info.into_inner(), &state.argon2_config).await {
        Ok(_) => {
            println!("Register new user {}", username);
            HttpResponse::Ok().json(RegisterResult::Successful)
        }
        Err(message) => {
            println!("Registration failed: {}", &message);
            HttpResponse::BadRequest().json(RegisterResult::Failed(format!("{}", message)))
        }
    }
}

pub async fn create_user(
    pool: &PgPool,
    register_info: RegisterInfo,
    config: &Config<'_>,
) -> Result<(), WebServerException> {
    let mut salt = vec![0u8; 64];
    OsRng.fill_bytes(&mut salt);

    let hashed_password = argon2::hash_encoded(register_info.password.as_bytes(), &salt, config)
        .map_err(|_| WebServerException::HashPassword)?;
    let new_user = NewUser {
        username: register_info.username,
        password: hashed_password,
        email: register_info.email,
    };

    UserRepository::insert(pool, new_user).await
}

#[post("/login")]
pub async fn login(info: web::Json<LoginInfo>, state: web::Data<AppState<'_>>) -> impl Responder {
    let matching_users = UserRepository::find_by_username(&state.pg_pool, &info.username).await;
    if let Err(e) = matching_users {
        return HttpResponse::InternalServerError().body(format!("{}", e));
    }
    let matching_users = matching_users.unwrap();

    println!(
        "Login request from {} with password {}",
        info.username, info.password
    );
    if matching_users.is_empty() {
        HttpResponse::Unauthorized().json(LoginResult::InvalidLogin {
            message: "Invalid username or password".to_string(),
        })
    } else if matching_users.len() > 1 {
        panic!("Found multiple matching usernames!")
    } else {
        let matching_user = &matching_users[0];
        println!("Found matching user {:?}", matching_user); //@TODO remove to avoid login hash
        let is_valid =
            argon2::verify_encoded(&matching_user.password, info.password.as_bytes()).unwrap();
        if is_valid {
            let mut raw_token = vec![0u8; 64];
            OsRng.fill_bytes(&mut raw_token);
            let token = base64::encode(&raw_token);
            state
                .tokens
                .write()
                .expect("Couldn't access the token storage")
                .insert(token.clone(), matching_user.id);
            println!("Saved token {} for user id {}.", &token, matching_user.id);
            let cookie = Cookie::build(AUTH_COOKIE_NAME, token)
                .max_age(Duration::days(31))
                .finish();
            HttpResponse::Ok()
                .cookie(cookie)
                .json(LoginResult::ValidLogin {
                    username: matching_user.username.to_string(),
                    user_id: matching_user.id,
                })
        } else {
            HttpResponse::Unauthorized().json(LoginResult::InvalidLogin {
                message: "Invalid username or password".to_string(),
            })
        }
    }
}

#[get("/login_status")]
pub async fn login_status(req: HttpRequest, state: web::Data<AppState<'_>>) -> impl Responder {
    match get_current_user(&req, &state).await {
        Ok(None) | Err(_) => web::Json(LoginState::LoggedOut),
        Ok(Some(user)) => web::Json(LoginState::LoggedIn {
            username: user.username,
            user_id: user.id,
        }),
    }
}

#[post("/logout")]
pub async fn logout() -> impl (Responder) {
    let mut cookie = Cookie::build(AUTH_COOKIE_NAME, "").finish();
    cookie.make_removal();
    HttpResponse::Ok().cookie(cookie).body("Logged out")
}

pub async fn get_current_user(
    req: &HttpRequest,
    state: &web::Data<AppState<'_>>,
) -> Result<Option<User>, WebServerException> {
    println!("Fetching the user from the cookies");
    let auth_token = if let Some(cookie) = req.cookie(AUTH_COOKIE_NAME) {
        String::from(cookie.value())
    } else {
        println!("Could not find the cookie in the headers");
        return Ok(None);
    };

    println!("Trying to find the user id for token {}", &auth_token);
    let user_id = if let Some(uid) = state
        .tokens
        .read()
        .expect("Couldn't access the token storage")
        .get(&auth_token)
    {
        *uid
    } else {
        println!("Could not find the token in the storage");
        return Ok(None);
    };

    println!("Trying to find user in database with uid {} ", user_id);
    let matching_users = UserRepository::find_by_id(&state.pg_pool, user_id).await;
    if matching_users.is_err() {
        println!("Failed to find user with uid {} in database", user_id);
        return Ok(None);
    }

    let matching_users = matching_users.unwrap();

    if matching_users.is_empty() {
        Err(WebServerException::User(
            "Token is valid but the user was not found in the database!".to_string(),
        ))
    } else if matching_users.len() > 1 {
        Err(WebServerException::User(
            "Found multiple users with the given id!".to_string(),
        ))
    } else {
        Ok(Some(matching_users[0].clone()))
    }
}
