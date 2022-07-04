use crate::{db, models::Category, schema::categories};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub fn configure(cfg: &mut web::ServiceConfig) {}

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
    diesel::insert_into(categories::dsl::categories).values(&category).execute(&conn).expect("Error posting");
    Ok(HttpResponse::Ok().body("Publish successfully"))
}
