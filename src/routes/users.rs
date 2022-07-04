// use diesel::ExpressionMethods;
// use diesel::QueryDsl;
// use diesel::RunQueryDsl;

// use crate::errors::AppError;
// use crate::{db, models};

// use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Result};
// use serde_derive::{Deserialize, Serialize};

// use crate::schema;
// use crate::schema::users::dsl::*;

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_five_users)
//         .service(get_user)
//         .service(create_user)
//         .service(put)
//         .service(destroy);
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct UserInput {
//     username: String,
// }

// #[post("/users")]
// pub async fn create_user(
//     db: web::Data<db::Pool>,
//     item: web::Json<models::User>,
// ) -> Result<HttpResponse, Error> {
//     let conn = db.get().unwrap();
//     let new_user = models::User {
//         username: item.username.to_string(),
//         email: item.email.to_string(),
//     };
//     diesel::insert_into(schema::users::dsl::users)
//         .values(&new_user)
//         .execute(&conn)
//         .expect("Error posting");
//     Ok(HttpResponse::Ok().body("Posted successfully"))
// }

// #[get("/users")]
// pub async fn get_five_users(db: web::Data<db::Pool>) -> Result<HttpResponse, AppError> {
//     let conn = db.get().unwrap();
//     let results = users
//         .limit(5)
//         .load::<(i32, String, String)>(&conn)
//         .expect("Error loading users");
//     Ok(HttpResponse::Ok().json(results))
// }

// #[get("/users/{id}")]
// pub async fn get_user(
//     db: web::Data<db::Pool>,
//     path: web::Path<i32>,
// ) -> Result<HttpResponse, AppError> {
//     let conn = db.get().unwrap();
//     let user_id = path.into_inner();
//     let results = users
//         .filter(schema::users::id.eq(user_id))
//         .load::<(i32, String, String)>(&conn)
//         .expect("Error finding users");
//     Ok(HttpResponse::Ok().json(results))
// }

// // Put API
// #[put("/users/{id}")]
// async fn put(
//     db: web::Data<db::Pool>,
//     path: web::Path<i32>,
//     item: web::Json<models::User>,
// ) -> Result<impl Responder> {
//     let user_id = path.into_inner();
//     let conn = db.get().unwrap();
//     let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(user_id));

//     diesel::update(target)
//         .set(schema::users::dsl::email.eq(item.email.to_string()))
//         .execute(&conn)
//         .expect("Error updating new post");

//     Ok(HttpResponse::Created().body("Update OK"))
// }

// #[delete("/users/{id}")]
// async fn destroy(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
//     let user_id = path.into_inner();
//     let conn = db.get().unwrap();
//     let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(user_id));

//     diesel::delete(target)
//         .execute(&conn)
//         .expect("Error deleting new post");

//     Ok(HttpResponse::Created().body("Delete Ok"))
// }
