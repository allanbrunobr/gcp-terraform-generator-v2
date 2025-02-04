mod handlers;
mod models;
mod services;
mod utils;

use axum::{routing::post, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", post(handlers::generate_terraform))
        .route("/migrate", post(handlers::migrate_resources))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Servidor rodando em http://127.0.0.1:3001");

    axum::serve(listener, app).await.unwrap();
}
