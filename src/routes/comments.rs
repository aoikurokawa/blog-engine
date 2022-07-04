// use crate::{
//     db, models,
//     schema::{comments, posts, users},
// };
// use actix_web::{get, post, web, Error, HttpResponse, Result};
// use diesel::prelude::*;
// use serde_derive::{Deserialize, Serialize};

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(create_comment)
//         .service(post_comments)
//         .service(user_comments);
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct CommentInput {
//     user_id: i32,
//     body: String,
// }

// #[post("/comment/{post_id}")]
// async fn create_comment(
//     db: web::Data<db::Pool>,
//     path: web::Path<i32>,
//     comment: web::Json<CommentInput>,
// ) -> Result<HttpResponse, Error> {
//     let conn = db.get().unwrap();
//     let post_id = path.into_inner();

//     let new_comment = models::Comment {
//         user_id: comment.user_id,
//         post_id: post_id,
//         body: comment.body.to_string(),
//     };

//     diesel::insert_into(comments::dsl::comments)
//         .values(&new_comment)
//         .execute(&conn)
//         .expect("Error posting");

//     Ok(HttpResponse::Ok().body("Create successfully."))
// }

// #[get("/comment/{post_id}")]
// async fn post_comments(
//     db: web::Data<db::Pool>,
//     path: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let conn = db.get().unwrap();
//     let post_id = path.into_inner();

//     let result = comments::table
//         .filter(comments::post_id.eq(post_id))
//         .inner_join(users::table)
//         .select((comments::all_columns, (users::id, users::username)))
//         .load::<((i32, i32, i32, String), (i32, String))>(&conn)
//         .expect("Failed to get comments");

//     Ok(HttpResponse::Ok().json(result))
// }

// #[get("/comment/{user_id}")]
// async fn user_comments(
//     db: web::Data<db::Pool>,
//     path: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     let conn = db.get().unwrap();
//     let user_id = path.into_inner();

//     let result = comments::table
//         .filter(comments::user_id.eq(user_id))
//         .inner_join(posts::table)
//         .select((
//             comments::all_columns,
//             (posts::id, posts::title, posts::body),
//         ))
//         .load::<((i32, i32, i32, String), (i32, String, String))>(&conn)
//         .expect("Failed to get comments");

//     Ok(HttpResponse::Ok().json(result))
// }

