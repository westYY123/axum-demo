use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use rdkafka::{producer::FutureProducer, ClientConfig};
use sea_orm::DatabaseConnection;

use crate::{model::init_conn, setting::AppConfig};

use self::{
    client::client,
    user::{delete_user, login},
};
use self::{health::health, produce_message::send_message};
use self::{put_get_redis::put_and_get_redis, user::register};
use redis::Client as RedisClient;
use reqwest::Client;

mod client;
mod health;
mod idl;
pub mod kafka_consumer;
mod produce_message;
mod put_get_redis;
mod user;

#[derive(Clone)]
pub struct AppData {
    http_client: Client,
    kafka_client: FutureProducer,
    mysql_client: DatabaseConnection,
    redis_client: RedisClient,
}

pub async fn create_route(config: AppConfig) -> Router {
    let producer: FutureProducer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            format!("{}:{}", &config.kafka.host, &config.kafka.port),
        )
        .set("message.timeout.ms", format!("{}", config.kafka.timeout))
        .create()
        .expect("Create kafka producer failed");
    let mysql = init_conn(config.mysql).await;
    let redis_client = redis::Client::open(format!("redis://{}/", config.redis.host))
        .map_err(|_| panic!("open redis failed"))
        .unwrap();

    let app_data = AppData {
        http_client: reqwest::Client::new(),
        kafka_client: producer,
        mysql_client: mysql,
        redis_client,
    };

    Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/register", post(register))
                .route("/login", post(login))
                .route("/delete", delete(delete_user)),
        )
        .route("/client", get(client))
        .route("/health", get(health))
        .route("/kafka/message", post(send_message))
        .route("/redis", post(put_and_get_redis))
        .with_state(Arc::new(app_data))
}
