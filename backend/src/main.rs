mod database;
mod models;
mod routes;

use axum::{
    routing::get,
    Json,
    Router,
};

use serde::Serialize;

use routes::products::product_routes;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "SmartStock AI",
    })
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = database::connect_database()
        .await
        .expect("Database connection failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Database migration failed");

    let app = Router::new()
        .route("/api/health", get(health_check))
        .merge(product_routes())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:3000"
    )
    .await
    .expect("Failed to bind address");

    println!("SmartStock AI running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}