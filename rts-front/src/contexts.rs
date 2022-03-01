use std::rc::Rc;

use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum LoginState {
    LoggedOut,
    LoggedIn { username: String, user_id: i32 },
    Checking,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoginAction {
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

pub type LoginContext = UseReducerHandle<LoginState>;
