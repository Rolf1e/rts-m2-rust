pub mod user {
    use crate::schema::users;

    #[derive(Debug, Queryable, Clone)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password: String,
        pub email: String,
    }

    #[derive(Insertable)]
    #[table_name = "users"]
    pub struct NewUser<'a> {
        pub username: &'a str,
        pub password: &'a str,
        pub email: &'a str,
    }
}

pub mod ai {
    use crate::schema::ais;

    #[derive(Queryable)]
    pub struct AI {
        pub id: i32,
        pub owner: i32,
        pub code: String,
    }

    #[derive(Insertable)]
    #[table_name = "ais"]
    pub struct NewAi<'a> {
        pub owner: i32,
        pub code: &'a str,
    }
}

pub mod game {
    use crate::schema::matchs;

    #[derive(Debug, Queryable)]
    pub struct MatchDo {
        pub id: i32,
        pub winner: i32,
        pub looser: i32,
        pub score_winner: i32,
        pub score_looser: i32,
    }

    use diesel::sql_types::{Integer, Text};

    #[derive(QueryableByName)]
    pub struct LeaderBoardRowDo {
        #[sql_type = "Text"]
        #[diesel(deserialize_as = "String")]
        pub username: String,
        #[sql_type = "Integer"]
        #[diesel(deserialize_as = "i32")]
        pub wins: i32,
        #[sql_type = "Integer"]
        #[diesel(deserialize_as = "i32")]
        pub looses: i32,
        #[sql_type = "Integer"]
        #[diesel(deserialize_as = "i32")]
        pub score: i32,
    }

    #[derive(Insertable)]
    #[table_name = "matchs"]
    pub struct NewMatchDo {
        pub winner: i32,
        pub looser: i32,
        pub score_winner: i32,
        pub score_looser: i32,
    }
}
