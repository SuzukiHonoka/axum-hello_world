use std::process;
use axum::{
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber;

const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    // start a thread for handle signal
    tokio::spawn(async move {
        handle_signal().await
    });

    // init tracing
    tracing_subscriber::fmt::init();

    // build app with a router
    let app = Router::new().route("/", get(root));

    let listener = match tokio::net::TcpListener::bind(BIND_ADDR).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to addr {}: {}", BIND_ADDR, e);
            return;
        }
    };

    println!("server will starts listening on {}", BIND_ADDR);
    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to serve: {}", e);
        }
    };
}

async fn root() -> &'static str {
    info!("Handling request to /");
    "Hello, world!"
}

async fn handle_signal() {
    #[cfg(target_os = "windows")]
    {
        use tokio::signal::windows::ctrl_c;
        let mut signal = ctrl_c().expect("Failed to listen for signal");
        signal.recv().await;
    }

    #[cfg(not(target_os = "windows"))]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to create SIGINT signal stream");
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to create SIGTERM signal stream");
        tokio::select! {
            _ = sigint.recv().await => (),
            _ = sigterm.recv().await => ()
        }
    }
    println!("Received signal, shutting down");
    process::exit(0);
}