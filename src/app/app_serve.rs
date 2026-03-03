use chrono::DateTime;
use redis::Commands;
use serde::{Deserialize, Serialize};

use std::sync::{Mutex, OnceLock};

use axum::{Router, response::Html, routing::get};
use log::info;

use crate::{app::config::config_env, business::model::AppInfoModel};

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
        APP_INFO_DATA
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .startup
            .format("%Y-%m-%d %H:%M:%S:%3f %:z")
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

pub fn run_poc() {
    // let i = fetch_an_integer().unwrap();
    // println!("{}", i);

    let data = fetch_an_counter().unwrap();
    println!("Counter value: {}", data.value);
    println!("Counter timestamp: {}", data.timestamp);
}

fn _fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open(config_env::get_config_env().redis_url.clone())?;
    let mut con = client.get_connection()?;

    // throw away the result, just make sure it does not fail
    let _: () = con.set("answer", 42)?;

    con.get("answer")
}

fn fetch_an_counter() -> redis::RedisResult<Counter> {
    let client = redis::Client::open(config_env::get_config_env().redis_url.clone())?;
    let mut con = client.get_connection()?;

    // throw away the result, just make sure it does not fail
    // let _: () = con.set("answer", 42)?;
    let data = Counter {
        value: 42,
        timestamp: chrono::Utc::now()
            .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).expect("Invalid offset")),
    };
    let data_json = serde_json::to_string(&data).expect("Failed to serialize Counter to JSON");
    let _: () = con.set("counter", data_json)?;

    //get counter
    let data_json: String = con.get("counter")?;
    let result: Counter =
        serde_json::from_str(&data_json).expect("Failed to deserialize JSON to Counter");
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Counter {
    pub value: i32,
    pub timestamp: DateTime<chrono::FixedOffset>,
}
