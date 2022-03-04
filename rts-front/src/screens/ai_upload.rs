use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::utils::alert_message;

enum AIUploadTabs {
    TypeIn,
    LoadPastebin,
    LoadGist,
}

#[derive(Serialize)]
enum AiSubmitRequest {
    Ai(String),
    PastebinKey(String),
    Gist { username: String, hash: String },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AiSubmitResponse {
    Successful,
    Failed(String),
}

enum ResultOrResponse {
    Result(AiSubmitResponse),
    Response(Response),
}

fn submit_ai(ai: &AiSubmitRequest, history: AnyHistory) {
    let post_body = serde_json::to_string(&ai).expect("Could not serialize request");
    wasm_bindgen_futures::spawn_local(async move {
        let response = match Request::post("/api/submit_ai")
            .header("Content-Type", "application/json")
            .body(post_body)
            .send()
            .await
        {
            Ok(response) => ResultOrResponse::Response(response),
            Err(err) => ResultOrResponse::Result(AiSubmitResponse::Failed(err.to_string())),
        };
        let login_result: AiSubmitResponse = match response {
            ResultOrResponse::Response(resp) => match resp.json().await {
                Ok(result) => result,
                Err(err) => AiSubmitResponse::Failed(err.to_string()),
            },
            ResultOrResponse::Result(result) => result,
        };
        match login_result {
            AiSubmitResponse::Successful => {
                history.push(Route::HomeScreen);
            }
            AiSubmitResponse::Failed(message) => {
                alert_message(&format!("Error submitting AI: {}", message))
            }
        }
    });
}

#[function_component(TypeInAITab)]
fn type_in_ai_tab() -> Html {
    let code = use_state(|| "".to_string());
    let history = use_history().expect("The history context is missing. Use this component in a BrowserRouter element.");

    let on_code_change = {
        let code = code.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            code.set(input.value());
        })
    };

    let on_submit = {
        let code = code.clone();
        Callback::from(move |_| {
            let history = history.clone();
            submit_ai(&AiSubmitRequest::Ai(code.to_string()), history);
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's code." }</p>
            <textarea onchange={on_code_change} value={(*code).clone()} cols="80" rows="30" required={true}/>
            <div>
                <button onclick={on_submit}>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(LoadPastebinAITab)]
fn load_pastebin_ai_tab() -> Html {
    let pastebin_key = use_state(|| "".to_string());
    let history = use_history().expect("The history context is missing. Use this component in a BrowserRouter element.");

    let on_pastebin_key_change = {
        let pastebin_key = pastebin_key.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            pastebin_key.set(input.value());
        })
    };

    let on_submit = {
        let pastebin_key = pastebin_key.clone();
        Callback::from(move |_| {
            let history = history.clone();
            submit_ai(
                &AiSubmitRequest::PastebinKey(pastebin_key.to_string()),
                history,
            );
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's Pastebin key." }</p>
            <label>{ "Key:" }<input onchange={on_pastebin_key_change} value={(*pastebin_key).clone()} required={true}/></label>
            <div>
                <button onclick={on_submit}>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(LoadGistAITab)]
fn load_gist_ai_tab() -> Html {
    let gist_username = use_state(|| "".to_string());
    let gist_hash = use_state(|| "".to_string());
    let history = use_history().expect("The history context is missing. Use this component in a BrowserRouter element.");

    let on_gist_username_change = {
        let gist_username = gist_username.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            gist_username.set(input.value());
        })
    };

    let on_gist_hash_change = {
        let gist_hash = gist_hash.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            gist_hash.set(input.value());
        })
    };

    let on_submit = {
        let gist_username = gist_username.clone();
        let gist_hash = gist_hash.clone();
        Callback::from(move |_| {
            let history = history.clone();
            submit_ai(
                &AiSubmitRequest::Gist {
                    username: gist_username.to_string(),
                    hash: gist_hash.to_string(),
                },
                history,
            );
        })
    };

    html! {
        <>
            <p>{ "Type in your AI's Gist username and hash." }</p>
            <div>
                <label>{ "Username:" }<input onchange={on_gist_username_change} value={(*gist_username).clone()} required={true}/></label>
            </div>
            <div>
                <label>{ "Hash:" }<input onchange={on_gist_hash_change} value={(*gist_hash).clone()} required={true}/></label>
            </div>
            <div>
                <button onclick={on_submit}>{"Submit AI"}</button>
            </div>
        </>
    }
}

#[function_component(AIUpload)]
pub fn ai_upload() -> Html {
    let selected_tab_index = use_state(|| AIUploadTabs::TypeIn);

    let select_type_in_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::TypeIn))
    };
    let select_pastebin_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::LoadPastebin))
    };
    let select_gist_tab = {
        let selected_tab_index = selected_tab_index.clone();
        Callback::from(move |_| selected_tab_index.set(AIUploadTabs::LoadGist))
    };

    html! {
        <>
            <h1>{ "AI Upload" }</h1>
            <button onclick={select_type_in_tab}>{ "Write down AI" }</button>
            <button onclick={select_pastebin_tab}>{ "Load from Pastebin" }</button>
            <button onclick={select_gist_tab}>{ "Load from Github Gist" }</button>
            <div>
                {
                    match *selected_tab_index {
                        AIUploadTabs::TypeIn => html! { <TypeInAITab/> },
                        AIUploadTabs::LoadPastebin => html! { <LoadPastebinAITab/> },
                        AIUploadTabs::LoadGist => html! { <LoadGistAITab/> },
                    }
                }
            </div>
        </>
    }
}
