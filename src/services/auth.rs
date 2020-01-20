use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    (cfg).route("/jwt", web::post().to(auth));
}

async fn auth(_req: HttpRequest) -> Option<impl Responder> {
    let success = false;
    if success {
        Some("ThisIsAJWT")
    } else {
        None
    }
}
