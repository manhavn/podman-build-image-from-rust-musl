mod handlers;
mod models;
mod routes;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Khởi tạo router gốc
    let app = Router::new()
        .route("/", get(|| async { "Hello from Rust API!" }))
        .nest("/api", routes::create_routes()); // mount các route con

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
