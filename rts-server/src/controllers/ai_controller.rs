use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::dto::input::AiInfo;
use crate::dto::output::AiResult;
use crate::models::ai::*;
use crate::repositories::ai_repo::AiRepository;
use crate::AppState;

use super::user_controller::get_current_user;

pub async fn fetch_ai_from_pastebin(paste_key: &str) -> Option<String> {
    reqwest::get(&format!(
        "https://pastebin.com/raw/{pastebin_key}",
        pastebin_key = paste_key
    ))
    .await
    .ok()?
    .text()
    .await
    .ok()
}

pub async fn fetch_ai_from_gist(username: &str, hash: &str) -> Option<String> {
    reqwest::get(&format!(
        "https://gist.githubusercontent.com/{username}/{hash}/raw",
        username = username,
        hash = hash
    ))
    .await
    .ok()?
    .text()
    .await
    .ok()
}

#[post("/submit_ai")]
pub async fn submit_ai(
    req: HttpRequest,
    state: web::Data<AppState<'_>>,
    info: web::Json<AiInfo>,
) -> impl (Responder) {
    // Authenticate the user
    let user = match get_current_user(&req, &state).await {
        Ok(None) => {
            return HttpResponse::Unauthorized()
                .json(AiResult::Failed("You are not logged in.".to_string()));
        }
        Err(e) => return HttpResponse::InternalServerError().body(format!("{}", e)),
        Ok(Some(user)) => user,
    };
    println!("Found an user matching the cookies");

    // Save the AI in the database
    println!("Ai submit request with ai {:?} for user {:?}", &info, &user);
    let code = match info.0 {
        AiInfo::Ai(c) => c,
        AiInfo::PastebinKey(key) => match fetch_ai_from_pastebin(&key).await {
            None => {
                return HttpResponse::ServiceUnavailable().json(AiResult::Failed(
                    "Could not fetch code from pastebin.".to_string(),
                ));
            }
            Some(c) => c,
        },
        AiInfo::Gist { username, hash } => match fetch_ai_from_gist(&username, &hash).await {
            None => {
                return HttpResponse::ServiceUnavailable().json(AiResult::Failed(
                    "Could not fetch code from gist.".to_string(),
                ));
            }
            Some(c) => c,
        },
    };

    let new_ai = NewAi {
        owner: user.id,
        code,
    };

    let owner = user.id;

    match AiRepository::insert(&state.pg_pool, new_ai).await {
        Ok(_) => {
            println!("Create AI for user: {}", owner);
            HttpResponse::Ok().json(AiResult::Successful)
        }
        Err(err) => {
            println!("Ai submit failed: {}", &err);
            HttpResponse::BadRequest().json(AiResult::Failed(err.to_string()))
        }
    }
}
