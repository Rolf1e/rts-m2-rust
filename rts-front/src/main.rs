mod contexts;
mod screens;

use reqwasm::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::contexts::*;
use crate::screens::login::Login;
use crate::screens::ai_upload::AIUpload;

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

#[function_component(Leaderboard)]
fn leaderboard() -> Html {
    html! {
        <h1>{ "Leaderboard!" }</h1>
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
