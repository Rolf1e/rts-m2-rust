use std::rc::Rc;

use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use weblog::console_log;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    HomeScreen,
    #[at("/login")]
    Login,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/upload")]
    AIUpload,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(HomeScreen)]
fn home_screen() -> Html {
    html! {
        <h1>{ "Home screen!" }</h1>
    }
}

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
fn login_screen() -> Html {
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
            <label>{ "Username:" }<input onchange={on_username_change} value={(*username).clone()}/></label>
            <label>{ "Password:" }<input onchange={on_password_change} value={(*password).clone()} type="password"/></label>
            <button onclick={on_submit}>{ "Login" }</button>
        </>
    }
}

#[function_component(Leaderboard)]
fn leaderboard() -> Html {
    html! {
        <h1>{ "Leaderboard!" }</h1>
    }
}

#[function_component(AIUpload)]
fn ai_upload() -> Html {
    html! {
        <h1>{ "AI Upload" }</h1>
    }
}

#[function_component(PageNotFound)]
fn page_not_found() -> Html {
    html! {
        <h1>{ "Page not found" }</h1>
    }
}

#[function_component(NavigationBar)]
fn navigation_bar() -> Html {
    let login_context = use_context::<LoginContext>().expect("no context found");
    html! {
        <nav>
            <ul>
                <li><Link<Route> to={Route::HomeScreen}>{ "Home" }</Link<Route>></li>
                <li><Link<Route> to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>></li>
                <li><Link<Route> to={Route::AIUpload}>{ "Upload AI" }</Link<Route>></li>
                <li><a href="https://github.com/Rolf1e/rts-m2-rust">{ "View on github" }</a></li>
                <li>{
                    match *login_context {
                        LoginState::Checking => html! { "Loading…" },
                        LoginState::LoggedOut => html! { <Link<Route> to={Route::Login}>{ "Log in" }</Link<Route>> },
                        LoginState::LoggedIn{ username: _, user_id: _ } => html! { <Link<Route> to={Route::HomeScreen}>{ "Log out" }</Link<Route>> },
                    }
                }</li>
            </ul>
        </nav>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::HomeScreen => html! { <HomeScreen/> },
        Route::Login => html! { <Login/> },
        Route::Leaderboard => html! { <Leaderboard/> },
        Route::AIUpload => html! { <AIUpload/> },
        Route::NotFound => html! { <PageNotFound/> },
    }
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(untagged)]
enum LoginState {
    LoggedOut,
    LoggedIn { username: String, user_id: i32 },
    Checking,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum LoginAction {
    Login { username: String, user_id: i32 },
    Logout,
}

impl Reducible for LoginState {
    type Action = LoginAction;

    fn reduce(self: Rc<Self>, action: LoginAction) -> Rc<Self> {
        match action {
            LoginAction::Login { username, user_id } => Self::LoggedIn { username, user_id }.into(),
            LoginAction::Logout => Self::LoggedOut.into(),
        }
    }
}

type LoginContext = UseReducerHandle<LoginState>;

#[function_component(App)]
fn app() -> Html {
    let login_state = use_reducer(|| LoginState::Checking);
    {
        let login_state = login_state.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let response = match Request::get("/api/login_status").send().await {
                        Ok(response) => Some(response),
                        Err(err) => {
                            let window = gloo_utils::window();
                            window
                                .alert_with_message(&format!("Error checking login: {}", err))
                                .unwrap();
                            login_state.dispatch(LoginAction::Logout);
                            None
                        }
                    };

                    match response {
                        Some(resp) => login_state.dispatch(match resp.json().await {
                            Ok(state) => state,
                            Err(err) => {
                                let window = gloo_utils::window();
                                window
                                    .alert_with_message(&format!("Error checking login: {}", err))
                                    .unwrap();
                                LoginAction::Logout
                            }
                        }),
                        None => (),
                    };
                });
                || ()
            },
            (),
        );
    }

    html! {
        <ContextProvider<LoginContext> context={login_state}>
            <BrowserRouter>
                <header>
                    <NavigationBar/>
                </header>
                <hr/>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <hr/>
                <footer>
                    <p>{ "Built in Rust with yew!" }</p>
                </footer>
            </BrowserRouter>
        </ContextProvider<LoginContext>>
    }
}

fn main() {
    yew::start_app::<App>();
}
