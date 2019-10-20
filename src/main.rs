#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, Responder};
use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use listenfd::ListenFd;
use serde::Deserialize;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Deserialize)]
struct AuthRequest {
    code: String,
}

fn auth_return_github(info: web::Query<AuthRequest>) -> impl Responder {
    format!("Your code is: {}", info.code)
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new().route(r"/auth_return_github", web::get().to(auth_return_github))
    });
    server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(l) => server.listen(l).unwrap(),
        None => server.bind("127.0.0.1:9000").unwrap(),
    };
    server.run().unwrap();
}
