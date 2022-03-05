use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use argon2::{self, Config};
use cookie::time::Duration;
use cookie::Cookie;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::rngs::OsRng;
use rand::RngCore;

use crate::dto::input::{LoginInfo, RegisterInfo};
use crate::dto::output::{LoginResult, LoginState, RegisterResult};
use crate::schema::*;
use crate::models::user::*; 
use crate::AppState;

const AUTH_COOKIE_NAME: &str = "_token";

pub fn create_user(
    conn: &PgConnection,
    register_info: RegisterInfo,
    config: &Config,
) -> Result<User, String> {
    let mut salt = vec![0u8; 64];
    OsRng.fill_bytes(&mut salt);

    let hashed_password =
        argon2::hash_encoded(register_info.password.as_bytes(), &salt, config).unwrap();
    let new_user = NewUser {
        username: &register_info.username,
        password: &hashed_password,
        email: &register_info.email,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
    {
        Ok(user) => Ok(user),
        Err(err) => Err(err.to_string()),
    }
}

#[post("/register")]
pub async fn register(
    info: web::Json<RegisterInfo>,
    state: web::Data<AppState<'_>>,
) -> impl Responder {
    let conn = state.pool.get().expect("Could not connect to the database");
    println!(
        "Register request from {} with password {} and mail {}",
        info.username, info.password, info.email
    );
    match create_user(&conn, info.into_inner(), &state.argon2_config) {
        Ok(user) => {
            println!("User registered with id {}", user.id);
            HttpResponse::Ok().json(RegisterResult::Successful)
        }
        Err(message) => {
            println!("Registration failed: {}", &message);
            HttpResponse::BadRequest().json(RegisterResult::Failed(message))
        }
    }
}

#[post("/login")]
pub async fn login(info: web::Json<LoginInfo>, state: web::Data<AppState<'_>>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let conn = state.pool.get().expect("Could not connect to the database");
    let matching_users = users
        .filter(username.eq(info.username.clone()))
        .load::<User>(&conn)
        .expect("Error loading users");
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
        println!("Found matching user {:?}", matching_user);
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
    match get_current_user(&req, &state) {
        None => web::Json(LoginState::LoggedOut),
        Some(user) => web::Json(LoginState::LoggedIn {
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

pub fn get_current_user(req: &HttpRequest, state: &web::Data<AppState<'_>>) -> Option<Box<User>> {
    use crate::schema::users::dsl::*;
    println!("Fetching the user from the cookies");
    // Read the authentication cookie
    let auth_token = match req.cookie(AUTH_COOKIE_NAME) {
        None => {
            println!("Could not find the cookie in the headers");
            return None;
        }
        Some(cookie) => cookie.value().to_string(),
    };
    println!("Trying to find the user id for token {}", &auth_token);
    // Try to find the token in the app state
    let user_id = match state
        .tokens
        .read()
        .expect("Couldn't access the token storage")
        .get(&auth_token)
    {
        None => {
            println!("Could not find the token in the storage");
            return None;
        }
        Some(uid) => *uid,
    };
    // Find the user in the database
    let conn = state.pool.get().expect("Could not connect to the database");
    let matching_users = users
        .filter(id.eq(user_id))
        .load::<User>(&conn)
        .expect("Error loading users");
    if matching_users.is_empty() {
        panic!("Token is valid but the user was not found in the database!")
    } else if matching_users.len() > 1 {
        panic!("Found multiple users with the given id!")
    } else {
        Some(Box::new(matching_users[0].clone()))
    }
}
