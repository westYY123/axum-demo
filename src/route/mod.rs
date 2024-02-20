use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

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
        .route("/delete", delete(delete_user))
        .route("/client", get(client))
        .route("/health", get(health))
        .with_state(Arc::new(app_data))
}
