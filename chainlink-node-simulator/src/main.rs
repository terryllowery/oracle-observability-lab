use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Init loging
    tracing_subscriber::fmt::init();

    // Router implementation
    let app = Router::new()
    .route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    tracing::info!("The great Oracle node it starting with the blue pill on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Start server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "The great Oracle node is healthy and ready to serve!")
}
