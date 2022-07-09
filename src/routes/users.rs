use crate::schema::users::dsl::*;
use crate::{
    db,
    models::{NewUser, User},
};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
async fn get_users(db: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    db: web::Data<db::Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[post("/users")]
pub async fn add_user(
    db: web::Data<db::Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/users/{id}")]
pub async fn delete_user(
    db: web::Data<db::Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_single_user(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn get_all_users(db: web::Data<db::Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = db.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

fn db_get_user_by_id(
    pool: web::Data<db::Pool>,
    user_id: i32,
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn add_single_user(
    db: web::Data<db::Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        first_name: &item.first_name,
        last_name: &item.last_name,
        email: &item.email,
        create_at: chrono::Local::now().naive_local(),
    };
    let res = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&conn)?;
    Ok(res)
}

fn delete_single_user(
    db: web::Data<db::Pool>,
    user_id: i32,
) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count = diesel::delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}
