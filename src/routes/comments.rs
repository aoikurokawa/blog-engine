use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CommentInput {
    user_id: i32,
    body: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/{id}/comments").route(web::get().to(user_comments)))
        .service(
            web::resource("/posts/{id}/comments")
                .route(web::post().to(add_comment))
                .route(web::get().to(post_comments)),
        );
}

async fn add_comment(
    post_id: web::Path<i32>,
    comment: web::Json<CommentInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        let data = comment.into_inner();
        let user_id = data.user_id;
        let body = data.body;
        models::create_comment(conn, user_id, post_id.into_inner(), body.as_str())
    })
    .await
    .map(|comment| HttpResponse::Created().json(comment))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn post_comments(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        models::post_comments(conn, post_id.into_inner())
    })
    .await
    .map(|comment| HttpResponse::Ok().json(comment))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn user_comments(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        models::user_comments(conn, user_id.into_inner())
    })
    .await
    .map(|comment| HttpResponse::Ok().json(comment))
    .map_err(|_| HttpResponse::InternalServerError())?)
}
