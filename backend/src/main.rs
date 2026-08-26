use axum::{ routing::get, Router };

#[tokio::main]
async fn main() {
    // initialize structured logging
    tracing_subscriber::fmt::init();

    // health checkpoint
    let app = Router::new().route("/health", get(health_check));

    // bind to localhost:3000 using 127.0.0.1
    let listener = tokio::net::TcpListener
        ::bind("127.0.0.1:3000").await
        .expect("Failed to bind to port 3000");

    tracing::info!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("Server crashed");
}

async fn health_check() -> &'static str {
    "OK"
}
