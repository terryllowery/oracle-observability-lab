use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::net::SocketAddr;
use std::sync::Arc;

mod metrics;
use metrics::NodeMetrics;

#[tokio::main]
async fn main() {
    // Init loging
    tracing_subscriber::fmt::init();

    // Create metrics
    let node_metrics = Arc::new(
        NodeMetrics::new("node_1").expect("Failed to create metrics")
    );

    // Clone for the endpoint handler
    let metrics_clone = Arc::clone(&node_metrics);

    // Router implementation
    let app = Router::new()
    .route("/health", get(health_check))
    .route("/metrics", get(move || metrics_endpoint(metrics_clone)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    tracing::info!("The great Oracle node it starting with the blue pill on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Start server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "The great Oracle node is healthy and ready to serve!")
}

async fn metrics_endpoint(metrics: Arc<NodeMetrics>) -> impl IntoResponse {
    match metrics.export() {
        Ok(body) => (StatusCode::OK, body),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error gathering metrics: {}", e)
        ),
    }
}
