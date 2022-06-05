#[macro_use]
extern crate diesel;
extern crate serde;

pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

use crate::routes::users::*;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::{env, io};

// pub type Result<T> = std::result::Result<T, std::io::Error>;
// type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // let app = db::Blog::new(8080);
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            // .wrap(middleware::Logger::default())
            .configure(routes::users::configure)
        // .service(get_five_users)
        // .service(get_user)
        // .service(create_user)
        // .service(put)
        // .service(destroy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
