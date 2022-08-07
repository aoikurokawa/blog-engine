use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize, Serializer};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Blog {
    id: uuid::Uuid,
    title: String,
    content: String,
    created_at: chrono::Utc,
}

pub async fn get_blogs(pool: web::Data<PgPool>) -> HttpResponse {
    match select_all_blogs(&pool).await {
        Ok(blogs) => HttpResponse::Ok().json(blogs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn select_all_blogs(pool: &PgPool) -> Result<Vec<Blog>, sqlx::Error> {
    let rows = sqlx::query!(Blog, r#"SELECT id, title, content, created_at from blogs"#,)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}
