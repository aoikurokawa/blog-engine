use crate::{db, models::Category, schema::categories};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(create_category)
        .service(update_category)
        .service(all_categories)
        .service(delete_post);
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryInput {
    name: String,
}

#[post("/category")]
async fn create_category(
    db: web::Data<db::Pool>,
    category: web::Json<CategoryInput>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let category = Category {
        name: category.name.clone(),
    };
    diesel::insert_into(categories::dsl::categories)
        .values(&category)
        .execute(&conn)
        .expect("Error posting");
    Ok(HttpResponse::Ok().body("Post successfully"))
}

#[put("/category/update/{category_id}")]
async fn update_category(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
    category: web::Json<CategoryInput>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let category_id = path.into_inner();
    let category = Category {
        name: category.name.clone(),
    };
    let target = categories::dsl::categories.filter(categories::dsl::id.eq(category_id));

    diesel::update(target)
        .set(categories::dsl::name.eq(category.name))
        .execute(&conn)
        .expect("Error updating category");
    Ok(HttpResponse::Ok().body("Update successfully"))
}

#[get("/categories")]
async fn all_categories(db: web::Data<db::Pool>) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();

    let result = categories::table
        .select(categories::all_columns)
        .load::<(i32, String)>(&conn)
        .expect("Failed to get");

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/category/delete/{category_id}")]
async fn delete_post(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let category_id = path.into_inner();
    let result = diesel::delete(categories::table.filter(categories::id.eq(category_id)))
        .execute(&conn)
        .expect("Error deleting");
    Ok(HttpResponse::Ok().json(result))
}
