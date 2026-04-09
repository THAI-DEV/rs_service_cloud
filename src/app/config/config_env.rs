use serde::Serialize;
use std::sync::OnceLock;

static LOAD_ONCE: OnceLock<ConfigEnv> = OnceLock::new();

pub fn get_config_env() -> &'static ConfigEnv {
    LOAD_ONCE.get_or_init(|| {
        dotenvy::dotenv().expect("Failed to load .env file");

        let redis_url = dotenvy::var("REDIS_URL").expect("REDIS_URL environment variable not set");

        let result = ConfigEnv { redis_url };
        let result_json = serde_json::to_string_pretty(&result).unwrap();

        println!("**** Loading Config Env **** : {result_json}");

        result
    })
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct ConfigEnv {
    pub redis_url: String,
}
