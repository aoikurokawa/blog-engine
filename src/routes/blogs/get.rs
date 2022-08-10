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

// #[derive(Deserialize)]
// pub struct BlogId {
//     blog_id: uuid::Uuid,
// }

pub async fn get_blog_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<(uuid::Uuid,)>,
) -> HttpResponse {
    let blog_id = path.into_inner();
    match select_blog_by_id(&pool, blog_id.0).await {
        Ok(blog) => HttpResponse::Ok().json(blog),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn select_blog_by_id(pool: &PgPool, blog_id: uuid::Uuid) -> Result<Blog, sqlx::Error> {
    let row = sqlx::query!(
        r#"SELECT id, title, content, created_at FROM blogs WHERE id = $1"#,
        blog_id
    )
    .fetch_one(pool)
    .await?;

    let selected_blog = Blog {
        id: row.id,
        title: row.title,
        content: row.content,
        created_at: row.created_at,
    };

    Ok(selected_blog)

    // if result.is_some() {
    //     let selected_blog: Blog = result.unwrap();
    //     Ok(Some(Blog {
    //         id: selected_blog.id,
    //         title: selected_blog.title,
    //         content: selected_blog.content,
    //         created_at: selected_blog.created_at,
    //     }))
    // } else {
    //     Ok(None)
    // }
}
