use crate::helpers::spawn_app;

#[tokio::test]
async fn post_category_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=DeFi";

    // Act
    let response = client
        .post(&format!("{}/category", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name FROM categories",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved blogs");
    assert_eq!(saved.name, "DeFi");
}

#[tokio::test]
async fn post_category_returns_a_400_for_invalid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=";

    // Act
    let response = client
        .post(&format!("{}/category", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn get_all_blogs_returns_a_200() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/categories", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    println!("{:?}", response);
    assert_eq!(200, response.status().as_u16());
}
