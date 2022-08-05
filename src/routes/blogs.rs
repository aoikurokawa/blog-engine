use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    title: String,
    content: String,
}

pub async fn post_blog(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Adding '{}' '{}' as a new blog", form.title, form.content);
    log::info!("Saving new blog details in the database");
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
        Ok(_) => {
            log::info!("New blog have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::info!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
