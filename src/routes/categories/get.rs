use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: uuid::Uuid,
    name: String,
}

pub async fn get_categories(pool: web::Data<PgPool>) -> HttpResponse {
    match select_all_categories(&pool).await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn select_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let rows = sqlx::query_as!(Category, "SELECT id, name FROM categories",)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
