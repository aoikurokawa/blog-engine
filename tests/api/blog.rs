use crate::helpers::spawn_app;
use defistory::configuration::{get_configuration, DatabaseSettings};
use defistory::startup::run;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;

#[tokio::test]
async fn post_blog_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "title=HelloWorld&content=<h1>Hello world</h1>";

    // Act
    let response = client
        .post(&format!("{}/blog", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT title, content FROM blogs",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved blogs");
    assert_eq!(saved.title, "HelloWorld");
    assert_eq!(saved.content, "<h1>Hello world</h1>")
}

#[tokio::test]
async fn post_blog_returns_a_400_for_invalid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "title=&content=<h1>Hello world</h1>";

    // Act
    let response = client
        .post(&format!("{}/blog", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}
