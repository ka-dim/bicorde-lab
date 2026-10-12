use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind HTTP listener");

    println!("Listening on http://{address}");

    axum::serve(listener, server::app())
        .await
        .expect("HTTP server failed");
}
