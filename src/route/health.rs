use axum::{extract::Request, response::IntoResponse};

pub async fn health(request: Request) -> impl IntoResponse {
    println!("{request:?}");
    "hello from axum"
}
