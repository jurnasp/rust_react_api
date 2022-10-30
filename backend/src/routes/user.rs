use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::Deserialize;
use rocket::State;
use sqlx::{Pool, Postgres, types::Uuid};
use crate::auth::Auth;

use crate::models::user::{NewUserData, UserResult};

#[get("/first_user")]
pub fn get_user() -> Result<Value, &'static str>  {
    Ok(json!(UserResult{name: "John".to_string(), email: "john@paris.fn".to_string(), hashed_password: "asdfg".to_string()}))
}

#[get("/user/<id>")]
pub async fn db_get_user(db: &State<Pool<Postgres>>, id: Uuid) -> Result<Value, &'static str>  {
    let user = crate::database::user::get_user(&db, id).await;
    let user_result = user.to_user_result();

    Ok(json!(user_result))
}

#[post("/user/<id>/set-password", data = "<password>")]
pub async fn post_set_password(db: &State<Pool<Postgres>>, id: Uuid, password: String) -> Result<Value, &'static str> {
    let user = crate::database::user::get_user(&db, id).await;
    crate::database::user::set_password(&db, user, &password).await;

    Ok(json!("success"))
}

#[derive(Deserialize)]
pub struct LoginUser {
    user: LoginUserData
}

#[derive(Deserialize)]
struct LoginUserData {
    email: Option<String>,
    password: Option<String>,
}

#[post("/user/login", format = "json", data = "<user>")]
pub async fn post_login(db: &State<Pool<Postgres>>, user: Json<LoginUser>) -> Result<Value, Value> {
    let user = user.into_inner().user;
    let email = user.email.unwrap();
    let password = user.password.unwrap();

    let secret = crate::auth::TOKEN_SECRET;

    crate::database::user::login(&db, &*email, &*password)
        .await
        .map(|user| json!({ "user": user.to_user_auth(&secret.to_string().into_bytes())}))
        .ok_or_else(|| json!({"email or password":  "is invalid"}))
}

#[get("/user")]
pub async fn get_authenticated_user(auth: Auth, db: &State<Pool<Postgres>>) -> Result<Value, &'static str> {
    let secret = crate::auth::TOKEN_SECRET;

    let user = crate::database::user::get_user(&db, auth.id).await;
    let user_result = user.to_user_auth(&secret.to_string().into_bytes());

    Ok(json!({"user": user_result}))
}

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[post("/user", format = "json", data = "<new_user>")]
pub async fn post_new_user(db: &State<Pool<Postgres>>, new_user: Json<NewUser>) -> Result<Value, &'static str> {
    let new_user = new_user.into_inner().user;

    let secret = crate::auth::TOKEN_SECRET;

    let user = crate::database::user::create(&db, new_user).await;
    let user_result = user.to_user_auth(&secret.to_string().into_bytes());

    Ok(json!({"user": user_result}))
}
