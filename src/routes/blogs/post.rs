use crate::domain::NewBlog;
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

    let new_blog = NewBlog {
        title: form.0.title,
        content: form.0.content,
    };

    if !new_blog.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    match insert_blog(&pool, &new_blog).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn insert_blog(pool: &PgPool, new_blog: &NewBlog) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO blogs (id, title, content, created_at)
            VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        new_blog.title,
        new_blog.content,
        chrono::Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| e)?;

    Ok(())
}
