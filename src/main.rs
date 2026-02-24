mod server_recorder;
mod structs;
mod utils;

use crate::structs::RecorderConfig;
use rust_mc_status::McClient;
use serde_json;
use sqlx::mysql::MySqlPoolOptions;
use std::fs;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let content = fs::read_to_string("config.json").expect("failed to read config.json");
    let config: structs::AppConfig =
        serde_json::from_str(content.as_str()).expect("failed to parse config.json");

    let db_url = config.db_url;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .min_connections(3)
        .connect(db_url.as_str())
        .await
        .expect("failed to connect to database");

    let mc_client = McClient::new()
        .with_timeout(Duration::from_millis(config.timeout as u64))
        .with_max_parallel(10);

    for server in config.servers {
        let internal_pool = pool.clone();
        let internal_client = mc_client.clone();
        tokio::spawn(async move {
            let args = RecorderConfig {
                interval: config.interval,
                mcserver: server,
                pool: internal_pool,
                client: internal_client,
            };
            server_recorder::server_recorder(args).await
        });
    }

    loop {
        tokio::time::sleep(Duration::from_secs(114514)).await;
    }
}
