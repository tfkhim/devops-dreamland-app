use axum::{Router, extract::MatchedPath, extract::Request};
use opentelemetry::trace::SpanKind;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub struct Middlewares;

impl Middlewares {
    pub fn apply_to<S>(router: Router<S>) -> Router<S>
    where
        S: Clone + Sync + Send + 'static,
    {
        let trace_layer = TraceLayer::new_for_http().make_span_with(move |request: &Request<_>| {
            let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
                matched_path.as_str()
            } else {
                request.uri().path()
            };

            let name = format!("{} {}", request.method(), path);

            tracing::span!(
                Level::INFO,
                "http_request",
                otel.name = name,
                otel.kind = ?SpanKind::Server,
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        });

        router.layer(trace_layer)
    }
}
