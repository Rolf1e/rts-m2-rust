use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
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
