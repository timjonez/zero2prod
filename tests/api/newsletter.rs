use crate::helpers::{spawn_app, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletters_are_not_sent_to_unconfirmed_subs() {
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter Title",
        "content": {
            "text": "The body as plain text",
            "html": "<p>The body as HTML</p>"
        }
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/newsletter", &app.address))
        .json(&newsletter_request_body)
        .send()
        .await
        .expect("Failde to execute request");

    assert_eq!(response.status().as_u16(), 200);
}

async fn create_unconfirmed_subscriber(app: &TestApp) {
    let body = "name=test&email=test%40test.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();
}