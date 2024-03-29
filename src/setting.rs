use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub mysql: MysqlConfig,
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub db: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KafkaConfig {
    pub host: String,
    pub port: u32,
    pub timeout: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u32,
}
