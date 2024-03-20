// src/services/user_service.rs}
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::users::jwt::Claims;
use crate::users::models::{LoginData, NewUser, RegisterData, User};
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;
use diesel::RunQueryDsl;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::time::SystemTime;

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
        public_key: hashed_password.to_string(),
        password: hashed_password,
    };

    // Insérez new_user dans la base de données
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}

pub fn login_user(
    conn: &mut MysqlConnection,
    login_data: LoginData,
) -> Result<(String, User), diesel::result::Error> {
    let user = users
        .select((id, username, public_key, password))
        .filter(username.eq(&login_data.username))
        .first::<User>(conn)?;

    match verify(&login_data.password, &user.password) {
        Ok(valid) => {
            if valid {
                let expiration = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + 60 * 60;

                let claims = Claims {
                    sub: user.id,
                    exp: expiration as usize,
                };

                let token = encode(
                    &Header::new(Algorithm::HS256),
                    &claims,
                    &EncodingKey::from_secret("mabitelaclefsecrete".as_ref()), // Utilisez une vraie clé secrète
                )
                .map_err(|_| diesel::result::Error::NotFound)?;

                Ok((token, user))
            } else {
                Err(diesel::result::Error::NotFound) // Ou une erreur personnalisée
            }
        }
        Err(_) => {
            Err(diesel::result::Error::NotFound) // Ou une autre erreur personnalisée
        }
    }
}
