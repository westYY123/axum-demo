use std::sync::Arc;

use crate::{error::AppError, middleware::jwt::TokenClaims};
use axum::{
    body::Body,
    extract::{Request, State},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

use super::AppData;

pub async fn client(
    _user: TokenClaims,
    State(app_data): State<Arc<AppData>>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    let resp = app_data
        .http_client
        .get("http://baidu.com")
        .send()
        .await
        .map_err(|_| AppError::InternalError)?;
    println!("{request:?}");
    println!("{resp:?}");
    let t = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(resp.bytes().await.unwrap()))
        .unwrap();
    Ok(t)
}
