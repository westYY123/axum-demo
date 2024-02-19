use axum::Json;

use crate::{
    error::{AppError, AppResult},
    middleware::jwt::{generate_jwt, TokenClaims},
    model::user::{self},
};

use super::idl::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

pub async fn register(Json(req): Json<RegisterRequest>) -> AppResult<Json<RegisterResponse>> {
    user::insert_user(req.username, req.password)
        .await
        .map(|_| Json(RegisterResponse { success: true }))
}

pub async fn login(Json(req): Json<LoginRequest>) -> AppResult<Json<LoginResponse>> {
    let user = user::get_user(&req.username)
        .await?
        .ok_or(AppError::WrongCombinationCredentials)?;
    if user.password != req.password {
        return Err(AppError::WrongCombinationCredentials);
    }
    let token = generate_jwt(&TokenClaims {
        username: user.username,
    })?;
    Ok(Json(LoginResponse {
        success: true,
        token: token,
    }))
}
