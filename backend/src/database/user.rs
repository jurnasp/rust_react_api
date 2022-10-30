use sqlx::{Pool, Postgres, types::Uuid};

use crate::models::user::User;



pub async fn get_user(db: &Pool<Postgres>, user_id: Uuid) -> User {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&*db)
        .await
        .ok()
        .unwrap();

    user
}