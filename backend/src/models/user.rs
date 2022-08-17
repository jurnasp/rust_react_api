use sqlx::FromRow;
use serde::Serialize;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub hashed_password: String
}

#[derive(Serialize)]
pub struct UserResult {
    pub name: String,
    pub email: String,
    pub hashed_password: String
}