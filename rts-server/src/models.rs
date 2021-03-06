pub mod user {

    #[derive(Debug, Clone, sqlx::FromRow)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password: String,
        pub email: String,
    }

    #[derive(Debug)]
    pub struct NewUser {
        pub username: String,
        pub password: String,
        pub email: String,
    }
}

pub mod ai {

    pub struct AI {
        pub id: i32,
        pub owner: i32,
        pub code: String,
    }

    #[derive(Debug)]
    pub struct NewAi {
        pub owner: i32,
        pub code: String,
    }
}

pub mod game {

    #[derive(Debug)]
    pub struct MatchDo {
        pub id: i32,
        pub game: i32,
        pub player: i32,
        pub score: i32,
    }

    #[derive(Debug, sqlx::FromRow)]
    pub struct LeaderBoardRowDo {
        pub name: String,
        pub total_score: i64,
        pub wins: i64,
        pub looses: i64,
    }

    #[derive(Debug)]
    pub struct NewMatchDo {
        pub game: i32,
        pub player: i32,
        pub score: i32,
        pub winner: bool,
    }
}
