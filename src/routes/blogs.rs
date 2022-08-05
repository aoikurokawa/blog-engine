use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    content: String,
}

pub async fn post_blog(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
            INSERT INTO blogs (id, title, content, created_at) 
            VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.title,
        form.content,
        chrono::Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn fetch_all_blogs() -> HttpResponse {
    HttpResponse::Ok().finish()
}
