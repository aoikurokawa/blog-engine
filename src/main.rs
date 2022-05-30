#[macro_use]
extern crate juniper;
extern crate env_logger;

// mod schema;
mod graphql_schema;

use std::io;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

