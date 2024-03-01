use std::sync::Arc;

use axum::{extract::State, Json};
use redis::Commands;
use tracing::info;

use crate::error::{AppError, AppResult};

use super::{idl::RedisData, AppData};

pub async fn put_and_get_redis(
    State(app_data): State<Arc<AppData>>,
    Json(req): Json<RedisData>,
) -> AppResult<Json<String>> {
    info!("Get a put and get redis request");
    let mut con = app_data
        .redis_client
        .get_connection()
        .map_err(|_| AppError::InternalError)?;
    con.set(req.key.clone(), req.value)
        .map_err(|_| AppError::InternalError)?;

    Ok(Json(con.get(req.key).map_err(|_| AppError::InternalError)?))
}
