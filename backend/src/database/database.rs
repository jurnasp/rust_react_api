use sqlx::{Pool, Postgres};

pub(crate) type Db = Pool<Postgres>;