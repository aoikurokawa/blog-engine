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
