use crate::util::token::Token;
use actix_web::{web, HttpRequest, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    (cfg).route("/jwt", web::post().to(auth));
}

async fn auth() -> impl Responder {
    Token {
        username: "Some OTHER Username".to_string(),
    }
    .to_jwt()
}
