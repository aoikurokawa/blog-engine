#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use actix_web::{middleware, App, HttpServer};
use diesel::pg::{self, ConnectionManager};
use diesel::prelude::*;

mod errors;
mod models;
mod routes;
mod schema;

type Pool = pg::Pool<ConnectionManager<PgConnection>>;

pub struct Blog {
    port: u16,
}
