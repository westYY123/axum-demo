use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use serde_json::Value;

// use crate::{error::AppError, model::{self, auth::Claims}};

// pub async fn login(Json(cred): Json<model::auth::User>) -> Result<Json<Value>, AppError> {
//     todo!()

// }
