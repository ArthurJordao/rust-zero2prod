mod fixtures;
use crate::fixtures::spawn_app;
use sqlx::PgPool;

#[sqlx::test]
async fn health_check_works(connection_pool: PgPool) {
    let app = spawn_app(connection_pool).await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
