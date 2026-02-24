use rust_mc_status::McClient;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MCServerConfig {
    pub address: String,
    pub times: u8,
    pub db_table: String,
}

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub interval: u32,
    pub timeout: u32,
    pub db_url: String,
    pub servers: Vec<MCServerConfig>,
}

#[derive(Clone)]
pub struct RecorderConfig {
    pub interval: u32,
    pub mcserver: MCServerConfig,
    pub pool: sqlx::MySqlPool,
    pub client: McClient,
}
