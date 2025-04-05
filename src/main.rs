use axum::{Router, routing::get};
use greetings_handler::greetings_handler;
use init::telemetry::TelemetryLifecycle;
use init::{InitResult, start_server};
use middleware::Middlewares;
use std::sync::Arc;
use user_repository::UserRepository;

mod greetings_handler;
mod init;
mod middleware;
mod user_repository;

#[tokio::main]
async fn main() -> InitResult<()> {
    let telemetry_lifecycle = TelemetryLifecycle::setup()?;

    start_server(build_app()).await?;

    telemetry_lifecycle.shutdown()?;

    Ok(())
}

fn build_app() -> Router {
    let user_repository = Arc::new(UserRepository::new());

    let router = Router::new()
        .route("/greetings/{user_id}", get(greetings_handler))
        .with_state(user_repository);

    Middlewares::apply_to(router)
}
