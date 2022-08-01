use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::Row;
use crate::database::{PgDatabase};

#[get("/db")]
pub async fn test_db(mut db: Connection<PgDatabase>) -> Result<Value, &'static str> {
    let result:i32 = sqlx::query("SELECT 1+1 AS result")
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.get("result")))
        .unwrap();

    Ok(json!(result))
}