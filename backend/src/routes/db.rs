use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::State;
use sqlx::{Row};
use crate::database::database::Db;

#[get("/db")]
pub async fn test_db(db: &State<Db>) -> Result<Value, &'static str> {
    let result: i32 = sqlx::query("SELECT 1+1 AS result")
        .fetch_one(&**db)
        .await
        .and_then(|r| Ok(r.get("result")))
        .unwrap();

    Ok(json!(result))
}