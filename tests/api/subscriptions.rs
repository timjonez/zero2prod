use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_200_valid_data() {
    let app = spawn_app().await;

    let body = "name=test&email=test%40test.com";
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "test@test.com");
    assert_eq!(saved.name, "test")
}

#[tokio::test]
async fn subscribe_returns_400_fields_present_but_empty() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=&email=test%40test.com", "empty name"),
        ("name=Test&email=", "empty email"),
        ("name=Test&email=not-a-email", "invalid email"),
    ];

    for (body, desc) in test_cases {
        let response = app.post_subscriptions(body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "the API did not return a 400 when the payload was {}",
            desc
        );
    }
}

#[tokio::test]
async fn subscribe_returns_400_invalid_data() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "Unexpected success with payload of: {}",
            error_message
        );
    }
}
