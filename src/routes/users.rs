use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use serde::{Deserialize, Serialize};

use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Result};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_by_id)
        .service(add_user)
        .service(delete_user);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    format!("hello from get users")
}

#[get("/users/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

#[post("/users")]
pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

#[delete("/users/{id}")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}
