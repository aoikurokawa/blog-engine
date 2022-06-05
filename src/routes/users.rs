use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::errors::AppError;
use crate::routes::convert;
use crate::{db::Pool, models::User};

use actix_web::{get, web, Error, HttpResponse, Responder, Result};
use futures::Future;
use serde_derive::{Deserialize, Serialize};

// use crate::errors::AppError;
use crate::schema::users;
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
pub async fn get_user(user_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<impl Responder> {
    let conn = &pool.get().unwrap();
    // format!("Hello {user_id}!")
    let id = user_id.into_inner();
    // let user = schema::users::table.select(schema::users)
    // let key = models::UserKey::ID(id);
    let user = users::table
        .select((users::id, users::username))
        .filter(users::id.eq(id))
        .load::<String>(&conn)
        .expect("error");
        // .first::<User>(conn);

    Ok(web::Json(user))
    // Ok(HttpResponse::Ok().json(user))
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
