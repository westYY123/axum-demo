use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::middleware::jwt::decode_user;

use self::health::health;
use self::register::register;
use self::client::client;
use reqwest::Client;
mod client;
mod health;
mod login;
mod register;

#[derive(Debug, Clone)]
pub struct AppData {
    http_client: Client,
}

pub fn create_route() -> Router {
    let protected_router = Router::new()
        .route("/health", get(health))
        .layer(middleware::from_fn(decode_user));
    let public_router = Router::new()
        .route("/register", post(register))
        .route("/client", get(client));
    let app_data = AppData {
        http_client: reqwest::Client::new(),
    };
    protected_router.merge(public_router).with_state(Arc::new(app_data))
}
