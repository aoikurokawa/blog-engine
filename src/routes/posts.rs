use crate::{
    db,
    models::{self, Post},
    schema::posts,
    schema::categories,
};
use actix_web::{get, post, put, web, Error, HttpResponse, Result};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(add_post)
        .service(publish_post)
        .service(category_posts)
        .service(all_posts);
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
        .and_then(|_| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;

            Ok(Post {
                user_id: id,
                title,
                body,
                published: false,
            })
        })
        .unwrap();
    diesel::insert_into(posts::dsl::posts)
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
    let target = posts::dsl::posts.filter(posts::dsl::id.eq(post_id));

    diesel::update(target)
        .set(posts::dsl::published.eq(true))
        .execute(&conn)
        .expect("Error updating new post");

    Ok(HttpResponse::Ok().body("Publish successfully"))
}

#[get("/post/{category_id}")]
async fn category_posts(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let category_id = path.into_inner();

    let result = posts::table
        .filter(posts::category_id.eq(category_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<(i32, i32, String, String, bool)>(&conn)
        .expect("Failed to get");

    Ok(HttpResponse::Ok().json(result))
}

#[get("/posts")]
async fn all_posts(db: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();

    let result = posts::table
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(categories::table)
        .select((posts::all_columns, (categories::id, categories::name)))
        .load::<((i32, i32, String, String, bool), (i32, String))>(&conn)
        .expect("Failed to get all posts");

    Ok(HttpResponse::Ok().json(result))
}
