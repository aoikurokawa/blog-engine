#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::{env, io};

pub type Result<T> = std::result::Result<T, std::io::Error>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();

    println!("Starting http server: 127.0.0.1:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
