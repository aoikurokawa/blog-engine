use crate::errors::AppError;
// use crate::routes::
use crate::{db, models, schema};
use actix_web::{post, put, web, Error, HttpResponse, Result};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
// use futures::Future;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_post).service(publish_post);
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

#[post("/post/{id}")]
async fn add_post(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
    post: web::Json<PostInput>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let post = models::find_user(&conn, id)
        .and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;

            Ok(models::Post {
                user_id: id,
                title,
                body,
                published: false,
            })
        })
        .unwrap();
    diesel::insert_into(schema::posts::dsl::posts)
        .values(&post)
        .execute(&conn)
        .expect("Error posting");
    Ok(HttpResponse::Ok().body("Posted successfully"))
}

#[put("/post/publish/{post_id}")]
async fn publish_post(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let post_id = path.into_inner();
    let target = schema::posts::dsl::posts.filter(schema::posts::dsl::id.eq(post_id));

    diesel::update(target)
        .set(schema::posts::dsl::published.eq(true))
        .execute(&conn)
        .expect("Error updating new post");

    Ok(HttpResponse::Ok().body("Publish successfully"))
}
