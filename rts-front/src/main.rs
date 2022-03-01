use reqwasm::http::Request;
use serde::Deserialize;
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

#[function_component(Login)]
fn login_screen() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

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
            // TODO
            console_log!(
                "Username:",
                (*username).clone(),
                "Password:",
                (*password).clone()
            )
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

#[function_component(App)]
fn app() -> Html {
    let login_state = use_state(|| LoginState::Checking);
    {
        let login_state = login_state.clone();
        use_effect_with_deps(
            move |_| {
                let login_state = login_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_state: LoginState = match Request::get("/api/login_status")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                    {
                        Ok(state) => state,
                        Err(err) => {
                            let window = gloo_utils::window();
                            window
                                .alert_with_message(&format!("Error checking login: {}", err))
                                .unwrap();
                            LoginState::LoggedOut
                        }
                    };
                    login_state.set(fetched_state);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <BrowserRouter>
            <header>
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::HomeScreen}>{ "Home" }</Link<Route>></li>
                        <li><Link<Route> to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>></li>
                        <li><Link<Route> to={Route::AIUpload}>{ "Upload AI" }</Link<Route>></li>
                        <li><a href="https://github.com/Rolf1e/rts-m2-rust">{ "View on github" }</a></li>
                        <li>{
                            match (*login_state).clone() {
                                LoginState::Checking => html! { "Loading…" },
                                LoginState::LoggedOut => html! { <Link<Route> to={Route::Login}>{ "Log in" }</Link<Route>> },
                                LoginState::LoggedIn{ username: _, user_id: _ } => html! { <Link<Route> to={Route::HomeScreen}>{ "Log out" }</Link<Route>> },
                            }
                        }</li>
                    </ul>
                </nav>
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
    }
}

fn main() {
    yew::start_app::<App>();
}
