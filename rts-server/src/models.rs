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

pub mod leader_board {
    use crate::schema::leader_board;

    #[derive(Queryable)]
    pub struct Rank {
        pub id: i32,
        pub rank: i32,
        pub player: i32,
        pub score: i32,
        pub wins: i32,
        pub losses: i32,
    }

    #[derive(Insertable)]
    #[table_name = "leader_board"]
    pub struct NewRank {
        pub rank: i32,
        pub player: i32,
        pub score: i32,
        pub wins: i32,
        pub losses: i32,
    }


}
