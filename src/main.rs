use std::io;

use axum::{extract::Path, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new().route("/greetings/:name", get(greet));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app).await
}

async fn greet<'de>(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
