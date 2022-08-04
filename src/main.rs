#[macro_use]
extern crate diesel;
extern crate serde;

pub mod auth;
pub mod db;
pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{dev::ServiceRequest, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::io;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.as_ref().clone())
        .unwrap_or_else(Default::default());

    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let pool = db::establish_connection();
    let host = std::env::var("HOST").expect("Host not set");
    let port = std::env::var("PORT").expect("Port not set");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .configure(routes::posts::configure)
            .configure(routes::categoris::configure)
            .configure(routes::users::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
