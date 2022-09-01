use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow)]
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

impl User {
    pub fn to_user_result(self) -> UserResult {
        UserResult {
            name: self.name,
            email: self.email,
            hashed_password: self.hashed_password,
        }
    }
}