use rocket_db_pools::{Connection, sqlx, sqlx::Row, sqlx::postgres::PgRow};
use crate::database::PgDatabase;
use crate::models::user::User;

pub async fn get_user(mut db: Connection<PgDatabase>, user_id: i32) -> User {
    let user = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .map(|row: PgRow| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            hashed_password: row.get("hashed_password"),
        })
        .fetch_one(&mut *db)
        .await
        .ok()
        .unwrap();


    user
}