use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Project {
    id: String,
    title: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    (cfg)
        .route("/get/{id}", web::get().to(get))
        .route("/list", web::get().to(list));
}

async fn get(req: HttpRequest) -> Option<impl Responder> {
    let id = req.match_info().get("id")?;
    Some(HttpResponse::Ok().json(Project {
        id: id.into(),
        title: "Some cool game".into(),
    }))
}

async fn list(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(vec![
        Project {
            id: "1234".into(),
            title: "Some cool game".into(),
        },
        Project {
            id: "1235".into(),
            title: "Some other cool game".into(),
        },
    ])
}
