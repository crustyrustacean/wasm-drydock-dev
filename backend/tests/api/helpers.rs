// tests/api/helpers.rs

use std::sync::LazyLock;
use wasm_drydock_dev_backend::configuration::get_configuration;
use wasm_drydock_dev_backend::startup::Application;
use wasm_drydock_dev_backend::telemetry::{get_subscriber, init_subscriber};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "info".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "info".into(), std::io::sink);
        init_subscriber(subscriber);
    };
});

#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.application.port = 0; // random port

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application.");
    let port = application.port();
    tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
    }
}
