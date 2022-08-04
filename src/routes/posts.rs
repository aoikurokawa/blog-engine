use crate::{db, errors::ServiceError, models::Post, schema::categories, schema::posts};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use chrono::prelude::*;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_post)
        .service(publish_post)
        .service(category_posts)
        .service(all_posts)
        .service(delete_post);
}

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
    category_id: i32,
}

#[post("/post")]
async fn create_post(
    db: web::Data<db::Pool>,
    post: web::Json<PostInput>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let post = Post {
        title: post.title.clone(),
        body: post.body.clone(),
        category_id: post.category_id,
        published: false,
        created: Utc::now().naive_utc(),
        updated: Utc::now().naive_utc(),
    };
    diesel::insert_into(posts::dsl::posts)
        .values(&post)
        .execute(&conn)
        .expect("Error posting");
    Ok(HttpResponse::Ok().body("Publish successfully"))
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
async fn category_posts(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let category_id = path.into_inner();

    let result = posts::table
        .filter(posts::category_id.eq(category_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<(i32, i32, String, String, bool, NaiveDateTime, NaiveDateTime)>(&conn)
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
        .load::<(
            (i32, i32, String, String, bool, NaiveDateTime, NaiveDateTime),
            (i32, String),
        )>(&conn)
        .expect("Failed to get all posts");

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/post/delete/{post_id}")]
async fn delete_post(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let conn = db.get().unwrap();
    let post_id = path.into_inner();
    diesel::delete(posts::table.filter(posts::id.eq(post_id))).execute(&conn)?;
    Ok(HttpResponse::Ok().body("Delete successfully"))
}
