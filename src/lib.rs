#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
