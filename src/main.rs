use std::{io, sync::Arc};

mod greetings_handler;
mod user_repository;

use axum::{routing::get, Router};
use greetings_handler::greetings_handler;
use tokio::net::TcpListener;
use user_repository::UserRepository;

#[tokio::main]
async fn main() -> io::Result<()> {
    let user_repository = Arc::new(UserRepository::new());

    let app = Router::new()
        .route("/greetings/:user_id", get(greetings_handler))
        .with_state(user_repository);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app).await
}
