use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::errors::AppError;
use crate::routes::convert;
use crate::{db, models};
use crate::{db::Pool, models::User};

use actix_web::{get, post, web, Error, HttpResponse, Responder, Result};
use diesel::prelude::*;
use futures::Future;
use serde_derive::{Deserialize, Serialize};

// use crate::errors::AppError;
use crate::schema;
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

// pub async fn create_user(
//     item: web::Json<UserInput>,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, Error> {
//     let conn = pool.get().await.map_err();
//     let username = item.into_inner().username;
//     let new_user = models::create_user(conn, username.as_str());
//     Ok(HttpResponse::Ok().json(new_user))
// }

// pub async fn find_user(
//     name: web::Path<String>,
//     pool: web::Data<Pool>,
// ) -> Result<HttpResponse, AppError> {
//     let conn = &pool.get().unwrap();
//     let name = name.into_inner();
//     let key = models::UserKey::Username(name.as_str());
//     let user = models::find_user(conn, key);
//     Ok(HttpResponse::Ok().json(user))
// }

#[get("/users/{id}")]
pub async fn get_user(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let user = schema::users::table
        .select(schema::users::email)
        .filter(schema::users::id.eq(id))
        .load::<String>(&conn)
        .expect("error");

    Ok(web::Json(user))
}

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

// Post API
#[post("/users")]
pub async fn post(
    db: web::Data<db::Pool>,
    item: web::Json<models::User>,
) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let new_user = models::User {
        // id: item.id as i32,
        email: item.email.to_string(),
    };
    diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new post");

    Ok(HttpResponse::Created().body("get ok"))
}
