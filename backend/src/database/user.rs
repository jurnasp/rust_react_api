use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::{Error, Row, postgres::PgRow, FromRow};

use crate::database::PgDatabase;
use crate::models::user::User;

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let id = row.try_get("id")?;
        let name = row.try_get("name")?;
        let email = row.try_get("email")?;
        let hashed_password = row.try_get("hashed_password")?;

        Ok(User{ id, name, email, hashed_password })
    }
}

pub async fn get_user(mut db: Connection<PgDatabase>, user_id: i32) -> User {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&mut *db)
        .await
        .ok()
        .unwrap();

    user
}