static KEY: &'static str = include_str!("../secret.key");

use jsonwebtoken::{decode, encode, errors::Result, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub username: String,
    pub exp: u64,
}

impl Token {
    pub fn from_jwt(jwt: &str) -> Result<Self> {
        decode::<Token>(jwt, KEY.as_ref(), &Validation::default()).map(|token| token.claims)
    }

    pub fn to_jwt(self) -> Option<String> {
        encode(&Header::default(), &self, KEY.as_ref()).ok()
    }
}
