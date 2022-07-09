#[macro_use]
extern crate diesel;
extern crate serde;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool = db::establish_connection();
    let host = std::env::var("HOST").expect("Host not set");
    let port = std::env::var("PORT").expect("Port not set");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::posts::configure)
            .configure(routes::categoris::configure)
            .configure(routes::users::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
