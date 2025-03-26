use axum::Router;
use tokio::net::TcpListener;

pub mod telemetry;

pub type InitResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn start_server(app: Router) -> InitResult<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
