use crate::util::token::Token;
use actix_web::{web, Responder};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn config(cfg: &mut web::ServiceConfig) {
    (cfg).route("/jwt", web::post().to(auth));
}

async fn auth() -> impl Responder {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    Token {
        username: "Some OTHER Username".to_string(),
        exp: since_the_epoch + 3600 * 12,
    }
    .to_jwt()
}
