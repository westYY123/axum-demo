use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use self::{client::client, user::login};
use self::health::health;
use self::user::register;
use reqwest::Client;
mod client;
mod health;
mod user;
mod idl;

#[derive(Debug, Clone)]
pub struct AppData {
    http_client: Client,
}

pub fn create_route() -> Router {
    let app_data = AppData {
        http_client: reqwest::Client::new(),
    };
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/client", get(client))
        .route("/health", get(health))
        .with_state(Arc::new(app_data))
}
