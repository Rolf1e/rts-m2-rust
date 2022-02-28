use yew::prelude::*;

enum Page {
    HomeScreen,
    Login,
    GamePlayback,
    Leaderboard,
    AIUpload,
}

#[function_component(HomeScreen)]
fn home_screen() -> Html {
    html! {
        <h1>{ "Home screen!" }</h1>
    }
}

#[function_component(Login)]
fn login_screen() -> Html {
    html! {
        <h1>{ "Login screen!" }</h1>
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

#[function_component(App)]
fn app() -> Html {
    let selected_page = use_state(|| Page::HomeScreen);
    let logged_in = use_state(|| false); // TODO set from cookies?

    html! {
        <>
            <header>
                <nav>
                    <ul>
                        <li><a href="#" onclick={let selected_page = selected_page.clone(); Callback::from(move |_| selected_page.set(Page::HomeScreen))}>{ "Home" }</a></li>
                        <li><a href="#" onclick={let selected_page = selected_page.clone(); Callback::from(move |_| selected_page.set(Page::Leaderboard))}>{ "View leaderboard" }</a></li>
                        <li><a href="#" onclick={let selected_page = selected_page.clone(); Callback::from(move |_| selected_page.set(Page::AIUpload))}>{ "Upload AI" }</a></li>
                        <li><a href="https://github.com/Rolf1e/rts-m2-rust">{ "View on github" }</a></li>
                        {
                            match *logged_in {
                                false => html! { <li><a href="#" onclick={let selected_page = selected_page.clone(); Callback::from(move |_| selected_page.set(Page::Login))}>{ "Login" }</a></li> },
                                true => html! { <li><a href="#" onclick={let selected_page = selected_page.clone(); Callback::from(move |_| selected_page.set(Page::HomeScreen))}>{ "Log out"}</a></li> },
                            }
                        }
                    </ul>
                </nav>
            </header>
            <hr/>
            <main>
                {
                    match *selected_page.clone() {
                        Page::HomeScreen => html! { <HomeScreen/> },
                        Page::Login => html! { <Login/> },
                        Page::Leaderboard => html! { <Leaderboard/> },
                        Page::AIUpload => html! { <AIUpload/> },
                        _ => html! { "Page not handled!" },
                    }
                }
            </main>
            <hr/>
            <footer>
                <p>{ "Built in Rust with yew!" }</p>
            </footer>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
