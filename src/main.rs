use std::io;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app).await
}
