use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;
use sqlx::{Pool, Postgres};

use crate::models::user::UserResult;

#[get("/first_user")]
pub fn get_user() -> Result<Value, &'static str>  {
    Ok(json!(UserResult{name: "John".to_string(), email: "john@paris.fn".to_string(), hashed_password: "asdfg".to_string()}))
}

#[get("/user/<id>")]
pub async fn db_get_user(db: &State<Pool<Postgres>>, id: i32) -> Result<Value, &'static str>  {
    let user = crate::database::user::get_user(&db, id).await;
    let user_result = user.to_user_result();

    Ok(json!(user_result))
}
