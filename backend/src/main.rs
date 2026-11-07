mod models;
mod routes;

use axum::{
    routing::get,
    Json,
    Router,
};

use serde::Serialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use routes::products::{product_routes, ProductStore};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_check() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "ok",
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store: ProductStore = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/api/health", get(health_check))
        .merge(product_routes().with_state(store));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://{}", address);
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}