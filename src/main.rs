#![feature(try_blocks)]

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;

mod middleware;
mod services;
mod util;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(crate::middleware::auth::Authentication)
            .service(web::scope("/auth").configure(crate::services::auth::config))
            .service(web::scope("/project").configure(crate::services::project::config))
            .service(web::scope("/").configure(crate::services::service_info::config))
            .route("/{name}", web::get().to(greet))
    });

    match listenfd.take_tcp_listener(0)? {
        Some(l) => server.listen(l)?,
        None => server.bind("127.0.0.1:7901")?,
    }
    .run()
    .await
}
