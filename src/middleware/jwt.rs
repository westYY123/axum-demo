use crate::error::AppError;
use axum::http::header;
use axum::{extract::Request, middleware::Next, response::IntoResponse};

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
const SECRET: &str = "fhriwhgruw";
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub async fn decode_user(mut req: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth| auth.to_str().ok())
        .and_then(|auth| {
            if auth.starts_with("Bearer") {
                Some(auth[7..].to_owned())
            } else {
                None
            }
        });
    let token = token.unwrap();
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::InternalError)?
    .claims;
    req.extensions_mut().insert(claims.sub);
    Ok(next.run(req).await)
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
}
