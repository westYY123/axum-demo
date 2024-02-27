use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};
use rdkafka::{producer::FutureProducer, ClientConfig};

use self::health::health;
use self::user::register;
use self::{
    client::client,
    user::{delete_user, login},
};
use reqwest::Client;
mod client;
mod health;
mod idl;
mod user;

#[derive(Clone)]
pub struct AppData {
    http_client: Client,
    kafka_client: FutureProducer,
}

pub fn create_route() -> Router {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Create kafka producer failed");
    let app_data = AppData {
        http_client: reqwest::Client::new(),
        kafka_client: producer,
    };
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/delete", delete(delete_user))
        .route("/client", get(client))
        .route("/health", get(health))
        .with_state(Arc::new(app_data))
}
