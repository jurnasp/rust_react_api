use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use serde::Serialize;

#[derive(Serialize)]
pub struct User{
    pub name: String,
    pub age: u8,
    pub address: String

}

#[get("/user")]
pub fn get_user() -> Result<Value, &'static str>  {
    Ok(json!(User{name: "John".to_string(), age: 22, address: "Paris".to_string()}))
}
