#[macro_use] extern crate rocket;
use serde::Serialize;
use rocket::serde::json::Json;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", routes![user])
}

#[get("/user")]
fn user() -> Json<User> {
    Json(User{name: "John".to_string(), age: 22, address: "Paris".to_string()})
}
