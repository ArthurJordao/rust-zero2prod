mod fixtures;
use crate::fixtures::spawn_app;
use reqwest::Response;
use sqlx::PgPool;

async fn create_subscription(body: String, app: &fixtures::TestApp) -> Response {
    app.http_client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.")
}

#[sqlx::test]
async fn subscribe_returns_a_200_for_valid_form_data(connection_pool: PgPool) {
    let app = spawn_app(connection_pool).await;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = create_subscription(body.into(), &app).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[sqlx::test]
async fn subscribe_returns_a_400_when_data_is_missing(connection_pool: PgPool) {
    let app = spawn_app(connection_pool).await;
    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = create_subscription(invalid_body.into(), &app).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[sqlx::test]
async fn subscribe_retuns_a_400_when_fields_are_present_but_empty(connection_pool: PgPool) {
    let app = spawn_app(connection_pool).await;
    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
    ];

    for (body, description) in test_cases {
        let response = create_subscription(body.into(), &app).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 200 OK when the payload was {}.",
            description
        );
    }
}
