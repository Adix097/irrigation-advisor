mod db;
mod domain;
mod infra;
mod models;
mod routes;

use axum::{ Router, routing::get };
use tower_http::cors::{ Any, CorsLayer };

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init(); // initialise logging

    // connect to Supabase
    let pool = db::create_pool().await;
    tracing::info!("database connected successfully");

    // cors
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    // routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/crops", get(routes::get_crops))
        .route("/api/soil-types", get(routes::get_soil_types))
        .route("/api/weather", get(routes::get_weather))
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener
        ::bind("127.0.0.1:3000").await
        .expect("Failed to bind to port 3000");
    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server crashed");
}

async fn health_check() -> &'static str {
    "OK"
}
