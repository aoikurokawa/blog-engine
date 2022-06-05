use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::errors::AppError;
use crate::routes::convert;
use crate::{db, models};
use crate::{db::Pool, models::User};

use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Result};
use diesel::prelude::*;
use futures::Future;
use serde_derive::{Deserialize, Serialize};

// use crate::errors::AppError;
use crate::schema;
use crate::schema::users::dsl::*;
// use diesel::prelude::*;
// use serde_derive::Serialize;

// pub async fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(web::resource("/users").route(web::post().to(create_user)))
//         .service(web::resource("/users/find/{name}").route(web::get().to(find_user)))
//         .service(web::resource("/users/{id}").route(web::get().to(get_user)));
// }

#[derive(Debug, Serialize, Deserialize)]
struct UserInput {
    username: String,
}

#[post("/users")]
pub async fn create_user(
    db: web::Data<db::Pool>,
    item: web::Json<models::User>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let new_user = models::User {
        username: item.username.to_string(),
        email: item.email.to_string(),
    };
    diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error posting");
    Ok(HttpResponse::Ok().body("Posted successfully"))
}

#[get("/users/{id}")]
pub async fn get_user(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();
    let results = users.filter(schema::users::id.eq(id)).limit(5).load::<User>(&conn).expect("Error loading users");
    Ok(HttpResponse::Ok().json(user))
}

// #[get("/users/{id}")]
// pub async fn get_user(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
//     let conn = db.get().unwrap();
//     let id = path.into_inner();
//     let user = schema::users::table
//         .select(schema::users::email)
//         .filter(schema::users::id.eq(id))
//         .load::<String>(&conn)
//         .expect("error");

//     Ok(web::Json(user))
// }

// pub fn find_user<'a>(conn: &PgConnection, key: UserKey<'a>) -> Result<User> {
//     match key {
//         UserKey::Username(name) => users::table
//             .filter(users::username.eq(name))
//             .select((users::id, users::username))
//             .first::<User>(conn)
//             .map_err(AppError::from),
//         UserKey::ID(id) => users::table
//             .find(id)
//             .select((users::id, users::username))
//             .first::<User>(conn)
//             .map_err(Into::into),
//     }
// }

// Put API
#[put("/users/{id}")]
async fn put(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
    item: web::Json<models::User>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let conn = db.get().unwrap();
    let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(id));

    diesel::update(target)
        .set(schema::users::dsl::email.eq(item.email.to_string()))
        .execute(&conn)
        .expect("Error updating new post");

    Ok(HttpResponse::Created().body("Update OK"))
}

#[delete("users/{id}")]
async fn destroy(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    let conn = db.get().unwrap();
    let target = schema::users::dsl::users.filter(schema::users::dsl::id.eq(id));

    diesel::delete(target)
        .execute(&conn)
        .expect("Error deleting new post");

    Ok(HttpResponse::Created().body("Delete Ok"))
}
