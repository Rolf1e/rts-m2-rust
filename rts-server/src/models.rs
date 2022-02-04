use crate::schema::{ais, users};

#[derive(Queryable)]
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
