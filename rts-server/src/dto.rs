pub mod output {

    use serde::{Serialize, Deserialize};

    #[derive(Serialize)]
    #[serde(untagged)]
    pub enum RegisterResult {
        Successful,
        Failed(String),
    }

    #[derive(Clone, PartialEq, Serialize)]
    #[serde(untagged)]
    pub enum LoginResult {
        ValidLogin { username: String, user_id: i32 },
        InvalidLogin { message: String },
    }

    #[derive(Clone, PartialEq, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum LoginState {
        LoggedOut,
        LoggedIn { username: String, user_id: i32 },
    }

    #[derive(Serialize)]
    #[serde(untagged)]
    pub enum AiResult {
        Successful,
        Failed(String),
    }
}

pub mod input {

    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct LoginInfo {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize)]
    pub struct RegisterInfo {
        pub username: String,
        pub password: String,
        pub email: String,
    }

    #[derive(Deserialize, Debug)]
    pub enum AiInfo {
        Ai(String),
        PastebinKey(String),
        Gist { username: String, hash: String },
    }
}
