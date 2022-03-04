use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::utils::alert_message;

#[derive(Serialize)]
struct RegistrationRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RegistrationResponse {
    Successful,
    Failed(String),
}

enum ResultOrResponse {
    Result(RegistrationResponse),
    Response(Response),
}

#[function_component(Registration)]
pub fn registration_screen() -> Html {
    let email = use_state(|| "".to_string());
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let password_confirm = use_state(|| "".to_string());
    let history = use_history().expect("The history context is missing. Use this component in a BrowserRouter element.");

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

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

    let on_password_confirm_change = {
        let password_confirm = password_confirm.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password_confirm.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let username = username.clone();
        let password = password.clone();
        // NOTE: The password should be equal to the password confirm when reaching here
        Callback::from(move |_| {
            let post_body = serde_json::to_string(&RegistrationRequest {
                email: email.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            })
            .expect("Could not serialize request");
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = match Request::post("/api/register")
                    .header("Content-Type", "application/json")
                    .body(post_body)
                    .send()
                    .await
                {
                    Ok(response) => ResultOrResponse::Response(response),
                    Err(err) => {
                        ResultOrResponse::Result(RegistrationResponse::Failed(err.to_string()))
                    }
                };
                let login_result: RegistrationResponse = match response {
                    ResultOrResponse::Response(resp) => match resp.json().await {
                        Ok(result) => result,
                        Err(err) => RegistrationResponse::Failed(err.to_string()),
                    },
                    ResultOrResponse::Result(result) => result,
                };
                match login_result {
                    RegistrationResponse::Successful => {
                        history.push(Route::Login);
                    }
                    RegistrationResponse::Failed(message) => {
                        alert_message(&format!("Error registering: {}", message))
                    }
                }
            });
        })
    };

    html! {
        <>
            <h1>{ "Registration screen!" }</h1>
            <div>
                <label>{ "Email:" }<input onchange={on_email_change} value={(*email).clone()} type="email" required={true}/></label>
            </div>
            <div>
                <label>{ "Username:" }<input onchange={on_username_change} value={(*username).clone()} required={true}/></label>
            </div>
            <div>
                <label>{ "Password:" }<input onchange={on_password_change} value={(*password).clone()} type="password" required={true}/></label>
            </div>
            <div>
                <label>{ "Confirm password:" }<input onchange={on_password_confirm_change} value={(*password_confirm).clone()} type="password" required={true}/></label>
            </div>
            <div>
                {
                    if (*password) == (*password_confirm) {
                        html! {
                            <button onclick={on_submit}>{ "Register" }</button>
                        }
                    } else {
                        html! {
                            <button disabled={true} title="Passwords don't match">{ "Register" }</button>
                        }
                    }
                }
            </div>
            <p>{ "Already have an account? " }<Link<Route> to={Route::Login}>{ "Log in!" }</Link<Route>></p>
        </>
    }
}
