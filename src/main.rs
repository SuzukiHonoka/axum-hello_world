use axum::{
    routing::get,
    Router,
};
use std::process;
use tokio::signal;
use tracing::info;
use tracing_subscriber;

const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // build app with a router
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to addr {}: {}", BIND_ADDR, e);
        process::exit(1);
    });

    println!("server will starts listening on {}", BIND_ADDR);
    if let Err(e) = axum::serve(listener, app).with_graceful_shutdown(handle_signal()).await {
        eprintln!("Failed to serve: {}", e);
    }
}

async fn root() -> &'static str {
    info!("Handling request to /");
    "Hello, world!"
}

async fn handle_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}