#[macro_use]
extern crate diesel;
extern crate serde;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::users::configure)
            .configure(routes::posts::configure)
            .configure(routes::comments::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
