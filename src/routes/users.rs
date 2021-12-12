use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use futures::future::Future;
// use futures_util::future::future::FutureExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

async fn create_user(
    item: web::Json<UserInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn = &pool.get().unwrap();
        let username = item.into_inner().username;
        models::create_user(conn, username.as_str())
    })
    .await.and_then(convert)
}

// fn create_user(
//     item: web::Json<UserInput>,
//     pool: web::Data<Pool>,
// ) -> impl Future<Output = HttpResponse> {
//     web::block(move || {
//         let conn = &pool.get().unwrap();
//         let username = item.into_inner().username;
//         models::create_user(conn, username.as_str())
//     })
//     .poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
// }