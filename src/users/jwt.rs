use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,   // ID de l'utilisateur
    pub exp: usize, // Expiration du token en secondes depuis l'époque UNIX
}
