use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let address = std::env::var("BICORDE_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());

    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind HTTP listener");

    println!("Listening on http://{address}");

    axum::serve(listener, server::app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("HTTP server failed");

    println!("HTTP server stopped cleanly");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C received; finishing active requests");
        }
        _ = terminate => {
            println!("SIGTERM received; finishing active requests");
        }
    }
}
