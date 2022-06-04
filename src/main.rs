#[macro_use]
extern crate diesel;
extern crate serde;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use db::Blog;
use dotenv::dotenv;
use std::{env, io};

pub type Result<T> = std::result::Result<T, std::io::Error>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix-web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app = db::Blog::new(8080);

    app.run(database_url).await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
