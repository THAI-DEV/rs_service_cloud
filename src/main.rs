use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    // let start_time = Instant::now();

    // let app_name = env!("CARGO_PKG_NAME"); // Get
    // let version = env!("CARGO_PKG_VERSION"); // Get the version from Cargo.toml

    // let build_date = env!("BUILD_DATE");
    // let build_time = env!("BUILD_TIME");
    // let build_date_time = format!("{build_date} {build_time} UTC+7");

    // let platform = helper::get_platform();
    // let rustc_version = env!("RUSTC_VERSION");

    // println!(
    // "--- (Begin {app_name} v{version} | Built with {rustc_version} on {platform} at {build_date_time}) ---",
    // );
    // println!("--- Begin---",);

    // println!();

    // app_server::run().await;

    // println!();

    // let (days, hours, minutes, seconds, millis) = calculate_elapsed_duration(start_time);

    // println!(
    //     "--- End ({days} days, {hours} hours, {minutes} minutes, {seconds} seconds, {millis} millis) ---"
    // );

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
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World! From DECH</h1>")
}
