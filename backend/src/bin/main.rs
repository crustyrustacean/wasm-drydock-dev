// src/bin/main.rs

use wasm_drydock_dev_backend::configuration::get_configuration;
use wasm_drydock_dev_backend::startup::Application;
use wasm_drydock_dev_backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber(
        "wasm-drydock-dev-backend".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
