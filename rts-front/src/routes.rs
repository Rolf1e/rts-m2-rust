use yew::prelude::*;
use yew_router::prelude::*;

use crate::screens::ai_upload::AIUpload;
use crate::screens::home_screen::HomeScreen;
use crate::screens::leaderboard::Leaderboard;
use crate::screens::login::Login;
use crate::screens::page_not_found::PageNotFound;
use crate::screens::register::Registration;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomeScreen,
    #[at("/login")]
    Login,
    #[at("/register")]
    Registration,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/upload")]
    AIUpload,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::HomeScreen => html! { <HomeScreen/> },
        Route::Login => html! { <Login/> },
        Route::Leaderboard => html! { <Leaderboard/> },
        Route::AIUpload => html! { <AIUpload/> },
        Route::Registration => html! { <Registration/> },
        _ => html! { <PageNotFound/> },
    }
}
