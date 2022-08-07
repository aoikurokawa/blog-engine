use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Blog {
    id: uuid::Uuid,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
}

pub async fn get_blogs(pool: web::Data<PgPool>) -> HttpResponse {
    match select_all_blogs(&pool).await {
        Ok(blogs) => HttpResponse::Ok().json(blogs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn select_all_blogs(pool: &PgPool) -> Result<Vec<Blog>, sqlx::Error> {
    let rows = sqlx::query_as!(Blog, "SELECT id, title, content, created_at FROM blogs",)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
