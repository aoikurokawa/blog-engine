use crate::domain::NewCategory;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
}

pub async fn post_category(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Adding '{}' as a new category", form.name);
    log::info!("Saving new category details in the database");

    let new_category = NewCategory { name: form.0.name };

    if !new_category.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    match insert_category(&pool, &new_category).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn insert_category(pool: &PgPool, new_blog: &NewCategory) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO categories (id, name)
            VALUES ($1, $2)
        "#,
        uuid::Uuid::new_v4(),
        new_blog.name,
    )
    .execute(pool)
    .await
    .map_err(|e| e)?;

    Ok(())
}
