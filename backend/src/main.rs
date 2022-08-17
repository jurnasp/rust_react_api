#[macro_use] extern crate rocket;

use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use rocket_db_pools::Database;

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
async fn main() {
    rocket::build()
        .attach(database::PgDatabase::init())
        .mount("/api", routes![routes::user::db_get_user, routes::user::get_user, routes::db::test_db])
        .register("/", catchers![not_found])
        .launch()
        .await;
}
