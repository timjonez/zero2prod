#[tokio::test]
async fn health_check_works() {
    spawn_app();
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind to address");

    let _ = tokio::spawn(server);
}
