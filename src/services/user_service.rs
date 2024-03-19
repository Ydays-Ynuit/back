// src/services/user_service.rs
// use crate::models::{NewUser, RegisterData};

use crate::schema::users;
use bcrypt::{hash, DEFAULT_COST};
use diesel::result::QueryResult;
use PgConnection;

pub fn create_user(conn: &PgConnection, register_data: RegisterData) -> QueryResult<NewUser> {
    let hashed_password = hash(register_data.password, DEFAULT_COST)?;
    let new_user = NewUser {
        username: register_data.username,
        password_hash: hashed_password,
    };

    // Insérez new_user dans la base de données
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}
