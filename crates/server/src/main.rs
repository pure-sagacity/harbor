use std::path::PathBuf;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

fn data_dir() -> PathBuf {
    std::env::var("HARBOR_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/var/lib/harbor"))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let address = {
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid u16");
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        format!("{}:{}", host, port)
    };

    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://{}", address);

    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
