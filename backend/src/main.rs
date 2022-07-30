#[macro_use] extern crate rocket;

use serde::Serialize;
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Database, Connection};
use rocket_db_pools::sqlx::Row;

#[derive(Serialize)]
struct User{
    pub name: String,
    pub age: u8,
    pub address: String

}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn user() -> Json<User> {
    Json(User{name: "John".to_string(), age: 22, address: "Paris".to_string()})
}

#[get("/db")]
async fn db(mut db: Connection<PgDatabase>) -> Result<Json<i32>, &'static str> {
    let result:i32 = sqlx::query("SELECT 1+1 AS result")
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.get("result")))
        .unwrap();

    Ok(Json(result))
}

#[derive(Database)]
#[database("pg_db")]
struct PgDatabase(sqlx::PgPool);

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rocket::build()
        .attach(PgDatabase::init())
        .mount("/", routes![index])
        .mount("/api", routes![user, db])
        .launch()
        .await;
}