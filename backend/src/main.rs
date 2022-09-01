#[macro_use] extern crate rocket;

use dotenv;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use sqlx::postgres::PgPoolOptions;

mod routes;
mod database;
mod models;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() -> Result<(), &'static str> {
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Unable to connect to database");

    rocket::build()
        .manage(pool)
        .mount("/api", routes![routes::user::db_get_user, routes::user::get_user, routes::db::test_db])
        .register("/", catchers![not_found])
        .launch()
        .await;

    Ok(())
}
