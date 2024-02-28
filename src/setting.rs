use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub mysql: MysqlConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
}
