use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::middleware::jwt::decode_user;

use self::health::health;
use self::register::register;

mod health;
mod login;
mod register;

pub fn create_route() -> Router {
    let protected_router = Router::new()
        .route("/health", get(health))
        .layer(middleware::from_fn(decode_user));
    let public_router = Router::new().route("/register", post(register));

    protected_router.merge(public_router)
}
