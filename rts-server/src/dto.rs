pub mod output {

    use serde::{Serialize, Deserialize};

    // === Leader Board ===

    #[derive(Serialize)]
    pub struct LeaderBoardResult {
        players: (PlayerResult, PlayerResult),

    }

    #[derive(Serialize)]
    pub struct PlayerResult {
        pub name: String,
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
}
