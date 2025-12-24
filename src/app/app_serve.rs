use std::sync::{Mutex, OnceLock};

use axum::{Router, response::Html, routing::get};
use log::info;

use crate::business::model::AppInfoModel;

static APP_INFO_DATA: OnceLock<Mutex<AppInfoModel>> = OnceLock::new();

#[tokio::main]
pub async fn run() {
    // Initialize tracing for logging
    log4rs::init_file("config_log.yaml", Default::default()).unwrap();

    init_app_info_data();

    // build our application with a route
    let routes = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    // println!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, routes).await.unwrap()

    let shutdown = async {
        info!("Press Ctrl+C to stop the server");

        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        info!(".. Shutting down server ...");
    };

    let server = axum::serve(listener, routes).with_graceful_shutdown(shutdown);

    server.await.unwrap();

    info!("Server has been shut down gracefully.");
}

async fn handler() -> Html<String> {
    let str = format!(
        "Hello, World From DECH (Start at : {})",
        APP_INFO_DATA.get().unwrap().lock().unwrap().startup
    );
    Html(str)
}

fn init_app_info_data() -> &'static Mutex<AppInfoModel> {
    APP_INFO_DATA.get_or_init(|| {
        Mutex::new(AppInfoModel {
            startup: chrono::Utc::now()
                .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).expect("Invalid offset")),
        })
    })
}
