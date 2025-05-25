mod config;
mod access;
mod proxy;
mod routes;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use config::{HOST, PORT};
use proxy::handler;
use routes::{index, robots};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建路由
    let app = Router::new()
        .route("/", get(index))
        .route("/robots.txt", get(robots))
        .route("/{*path}", get(handler).post(handler))
        .layer(CorsLayer::permissive());

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();

    println!("Server running on http://{}:{}", HOST, PORT);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}