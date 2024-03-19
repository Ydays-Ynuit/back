// auth.rs
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (whom the token refers to)
    pub exp: usize,  // Expiry
                     // Add other claims as needed
}

pub fn create_token(username: &str) -> String {
    let expiration = 86000; // 24 hours
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}
