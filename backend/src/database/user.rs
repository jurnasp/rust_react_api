use scrypt::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use scrypt::password_hash::rand_core::OsRng;
use scrypt::Scrypt;
use sqlx::{types::Uuid};
use sqlx::postgres::PgQueryResult;

use crate::database::database::Db;
use crate::models::user::{NewUser, NewUserData, User};


pub async fn get_user(db: &Db, user_id: Uuid) -> User {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&*db)
        .await
        .ok()
        .unwrap();

    user
}


pub async fn get_user_by_email(db: &Db, email: &str) -> User {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(&*db)
        .await
        .ok()
        .unwrap();

    user
}

pub async fn set_password(db: &Db, user: User, new_password: &str) -> PgQueryResult {
    let result = sqlx::query("UPDATE users SET hashed_password = $1 WHERE id = $2")
        .bind(new_password).bind(user.id)
        .execute(&*db)
        .await
        .ok()
        .unwrap();

    result
}

pub async fn login(db: &Db, email: &str, password: &str) -> Option<User> {
    let user = self::get_user_by_email(&db, email).await;

    let parsed_hash = PasswordHash::new(&user.hashed_password).unwrap();
    let password_matches = Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| eprintln!("login_user: scrypt_check: {}", err))
        .is_ok();

    if password_matches {
        Some(user)
    } else {
        eprintln!(
            "login attempt for '{}' failed: password doesn't match",
            email
        );
        None
    }
}

pub async fn create(db: &Db, user: NewUserData) -> User {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Scrypt
        .hash_password(user.password.unwrap().as_bytes(), &salt)
        .expect("hash error")
        .to_string()
        .to_owned();

    let insert_data = NewUser {
        name: &*user.username.unwrap(),
        email: &*user.email.unwrap(),
        hashed_password: &hash[..]
    };
    sqlx::query("INSERT INTO users(name, email, hashed_password) VALUES($1, $2, $3)")
        .bind(insert_data.name).bind(insert_data.email).bind(insert_data.hashed_password)
        .execute(&*db)
        .await.unwrap();
    let user = get_user_by_email(&db, insert_data.email).await;

    user
}
