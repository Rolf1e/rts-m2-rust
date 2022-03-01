use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;

use crate::contexts::*;

#[derive(Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(untagged)]
enum LoginResult {
    ValidLogin { username: String, user_id: i32 },
    InvalidLogin { message: String },
}

enum ResultOrResponse {
    Result(LoginResult),
    Response(Response),
}

#[function_component(Login)]
pub fn login_screen() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let login_context = use_context::<LoginContext>().expect("no context found");

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        Callback::from(move |_| {
            console_log!(
                "Username:",
                (*username).clone(),
                "Password:",
                (*password).clone()
            );
            let login_context = login_context.clone();
            let post_body = serde_json::to_string(&LoginRequest {
                username: username.to_string(),
                password: password.to_string(),
            })
            .expect("Could not serialize request");
            wasm_bindgen_futures::spawn_local(async move {
                let response = match Request::post("/api/login")
                    .header("Content-Type", "application/json")
                    .body(post_body)
                    .send()
                    .await
                {
                    Ok(response) => ResultOrResponse::Response(response),
                    Err(err) => {
                        let window = gloo_utils::window();
                        window
                            .alert_with_message(&format!("Error logging in: {}", err))
                            .unwrap();
                        ResultOrResponse::Result(LoginResult::InvalidLogin {
                            message: err.to_string(),
                        })
                    }
                };
                let login_result: LoginResult = match response {
                    ResultOrResponse::Response(resp) => match resp.json().await {
                        Ok(result) => result,
                        Err(err) => {
                            let window = gloo_utils::window();
                            window
                                .alert_with_message(&format!("Error logging in: {}", err))
                                .unwrap();
                            LoginResult::InvalidLogin {
                                message: err.to_string(),
                            }
                        }
                    },
                    ResultOrResponse::Result(result) => result,
                };
                match login_result {
                    LoginResult::ValidLogin { username, user_id } => {
                        login_context.dispatch(LoginAction::Login { username, user_id })
                    }
                    LoginResult::InvalidLogin { message: _ } => {
                        login_context.dispatch(LoginAction::Logout)
                    }
                }
            });
        })
    };

    html! {
        <>
            <h1>{ "Login screen!" }</h1>
            <div>
                <label>{ "Username:" }<input onchange={on_username_change} value={(*username).clone()}/></label>
            </div>
            <div>
                <label>{ "Password:" }<input onchange={on_password_change} value={(*password).clone()} type="password"/></label>
            </div>
            <div>
                <button onclick={on_submit}>{ "Login" }</button>
            </div>
        </>
    }
}
