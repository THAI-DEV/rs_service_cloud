use axum::{Router, response::Html, routing::get};

pub async fn run() {
    let routes = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();

    let shutdown = async {
        // info!("Press Ctrl+C to stop the server");

        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        // info!(".. Shutting down server ...");
    };

    let server = axum::serve(listener, routes).with_graceful_shutdown(shutdown);

    server.await.unwrap();

    // info!("Server has been shut down gracefully.");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World! From DECH</h1>")
}
