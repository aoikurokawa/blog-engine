use crate::{models, Pool};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to(add_post))
            .route(web::get().to(user_post)),
    )
    .service(web::resource("/posts").route(web::get().to(all_posts)))
    .service(web::resource("/posts/{id}/publish").route(web::post().to(publish_post)));
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

async fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner());
        models::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;
            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .await
    .map(|post| HttpResponse::Created().json(post))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        models::publish_post(conn, post_id.into_inner())
    })
    .await
    .map(|post| HttpResponse::Ok().json(post))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn user_post(user_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        models::user_posts(conn, user_id.into_inner())
    })
    .await
    .map(|post| HttpResponse::Ok().json(post))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn all_posts(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || {
        let conn = &pool.get().unwrap();
        models::all_posts(conn)
    })
    .await
    .map(|post| HttpResponse::Ok().json(post))
    .map_err(|_| HttpResponse::InternalServerError())?)
}
