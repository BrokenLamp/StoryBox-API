use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/info", web::get().to(info));
}

async fn info(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("service_info.html"))
}
