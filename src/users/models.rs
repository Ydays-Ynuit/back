use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub public_key: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
