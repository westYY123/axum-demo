use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    error::{AppError, AppResult},
    middleware::jwt::{generate_jwt, TokenClaims},
    model::user::{self},
};

use super::{
    idl::{DeleteUserResponse, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
    AppData,
};

pub async fn register(
    State(app_data): State<Arc<AppData>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    user::insert_user(&app_data.mysql_client, req.username, req.password)
        .await
        .map(|_| Json(RegisterResponse { success: true }))
}

pub async fn login(
    State(app_data): State<Arc<AppData>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let user = user::get_user(&app_data.mysql_client, &req.username)
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
        token,
    }))
}

pub async fn delete_user(
    State(app_data): State<Arc<AppData>>,
    claim: TokenClaims,
) -> AppResult<Json<DeleteUserResponse>> {
    println!("{:?}", claim.username);
    user::delete_user(&app_data.mysql_client, &claim.username)
        .await
        .map(|_| Json(DeleteUserResponse { success: true }))
}
