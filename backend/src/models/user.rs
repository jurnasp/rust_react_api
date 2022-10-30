use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::Uuid};
use crate::auth::Auth;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String
}

pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub hashed_password: &'a str,
}

#[derive(Deserialize)]
pub struct NewUserData {
    pub(crate) username: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResult {
    pub name: String,
    pub email: String,
    pub hashed_password: String
}

#[derive(Serialize)]
pub struct UserAuth {
    name: String,
    email: String,
    token: String,
}

impl User {
    pub fn to_user_result(self) -> UserResult {
        UserResult {
            name: self.name,
            email: self.email,
            hashed_password: self.hashed_password,
        }
    }

    pub fn to_user_auth(self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(60); // TODO: move to config
        let token = Auth {
            id: self.id,
            username: self.name.clone(),
            exp: exp.timestamp(),
        }
            .token(secret);

        UserAuth {
            name: self.name,
            email: self.email,
            token
        }
    }
}
