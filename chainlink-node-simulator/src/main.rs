use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;

mod metrics;
mod node;

use metrics::NodeMetrics;
use node::{OracleNode, QualityTier};

#[tokio::main]
async fn main() {
    // Init loging
    tracing_subscriber::fmt::init();

    // Read configuration from environment
    let node_id = std::env::var("NODE_ID")
    .unwrap_or_else(|_| "node_1".to_string());

    let quality_tier = std::env::var("QUALITY_TIER")
    .unwrap_or_else(|_| "high".to_string());

    let quality = match quality_tier.as_str() {
        "high" => QualityTier::High,
        "medium" => QualityTier::Medium,
        "low" => QualityTier::Low,
        _ => {
            tracing::warn!("Invalid QUALITY_TIER '{}', defaulting to 'high'", quality_tier);
            QualityTier::High
        }
    };

    let node_metrics = Arc::new(NodeMetrics::new(&node_id).expect("Failed to create metrics"));

    // Create Oracle node
    let node = Arc::new(OracleNode::new(
        node_id.clone(),
        quality,
        Arc::clone(&node_metrics),
    ));

    node.start_request_loop().await;

    tracing::info!("Starting simulation node {}", node_id);

    // Clone for the endpoint handler

    let metrics_clone = Arc::clone(&node_metrics);

    // Router implementation
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(move || metrics_endpoint(metrics_clone)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    tracing::info!(
        "The great Oracle node it starting with the blue pill on {}",
        addr
    );
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Start server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        "The great Oracle node is healthy and ready to serve!",
    )
}

async fn metrics_endpoint(metrics: Arc<NodeMetrics>) -> impl IntoResponse {
    match metrics.export() {
        Ok(body) => (StatusCode::OK, body),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error gathering metrics: {}", e),
        ),
    }
}
