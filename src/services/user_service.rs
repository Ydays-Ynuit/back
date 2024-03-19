// src/services/user_service.rs}
use crate::schema::users;
use crate::users::models::{NewUser, RegisterData};
use bcrypt::{hash, DEFAULT_COST};
use diesel::mysql::MysqlConnection;
use diesel::result::QueryResult;
use diesel::RunQueryDsl;

pub fn create_user(
    conn: &mut MysqlConnection,
    register_data: RegisterData,
) -> QueryResult<NewUser> {
    let hashed_password = match hash(register_data.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return Err(diesel::result::Error::NotFound), // Gérer l'erreur comme vous le souhaitez
    };

    let new_user = NewUser {
        username: register_data.username,
        public_key: "".to_string(),
        password: hashed_password,
    };

    // Insérez new_user dans la base de données
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}
