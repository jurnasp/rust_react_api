pub(crate) mod user;

use rocket_db_pools::{Database, sqlx};

#[derive(Database)]
#[database("pg_db")]
pub struct PgDatabase(sqlx::PgPool);
