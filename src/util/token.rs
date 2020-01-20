static KEY: Vec<u8> = *include_bytes!("../secret.key").into::<Vec<u8>>();

use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    username: String,
    exp: usize,
}

impl Token {
    pub fn from_jwt(jwt: String) -> Option<Token> {
        let token = decode::<Token>(&jwt, KEY.as_ref(), &Validation::default()).ok()?;
        Some(token.claims)
    }
    pub fn to_jwt(self) -> Option<String> {
        encode(&Header::default(), &self, KEY.as_ref()).ok()
    }
}
