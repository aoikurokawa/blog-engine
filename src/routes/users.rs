use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse, Responder};
use futures::future::Future;
// use futures_util::future::future::FutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

// async fn create_user(
//     item: web::Json<UserInput>,
//     pool: web::Data<Pool>,
// ) -> impl Future<Item = HttpResponse, Error = AppError> {
//     web::block(move || {
//         let conn = &pool.get().unwrap();
//         let username = item.into_inner().username;
//         models::create_user(conn, username.as_str())
//     })
//     .then(|res| {
//         res.map(|d| HttpResponse::Ok().json(d)).map_err(Into::into)
//     })
// }

fn create_user(item: web::Json<UserInput>, pool: web::Data<Pool>) -> impl Responder {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let username = item.into_inner().username;
        models::create_user(conn, username.as_str())
    })
    .then(convert)
    // .poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
}

fn find_user(name: web::Path<String>, pool: web::Data<Pool>) -> impl Responder {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let name = name.into_inner();
        let key = models::UserKey::USERNAME(name.as_str());
        models::find_user(conn, key)
    })
    .then(convert)
}

fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> impl Responder {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let id = user_id.into_inner();
        let key = models::UserKey::ID(id);
        models::find_user(conn, key)
    })
    .then(convert)
}
