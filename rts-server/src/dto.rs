pub mod output {

    use serde::{Deserialize, Serialize};

    // === Leader Board ===

    #[derive(Debug, Serialize)]
    pub struct LeaderBoardDto {
        pub username: String,
        pub score: i32,
        pub wins: i32,
        pub looses: i32,
    }

    // ===  User ===

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

    // === AI ===

    #[derive(Serialize)]
    #[serde(untagged)]
    pub enum AiResult {
        Successful,
        Failed(String),
    }
}

pub mod input {

    use serde::Deserialize;

    // === User ===

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

    // === AI ===

    #[derive(Deserialize, Debug)]
    pub enum AiInfo {
        Ai(String),
        PastebinKey(String),
        Gist { username: String, hash: String },
    }

    #[derive(Deserialize)]
    pub struct NewMatchDto {
        pub game: i32,
        pub player: i32,
        pub score: i32,
    }

}
