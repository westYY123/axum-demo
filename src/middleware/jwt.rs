use crate::error::{AppError, AppResult};
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &str = "fhriwhgruw";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub username: String,
    pub exp: usize,
}

#[async_trait]
impl<B> FromRequestParts<B> for TokenClaims
where
    B: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &B) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::MissingCredentials)?;
        println!("{}", bearer.token());
        let token_data = decode::<TokenClaims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::MissingCredentials)?;
        Ok(token_data.claims)
    }
}

pub fn generate_jwt(claims: &TokenClaims) -> AppResult<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .map_err(|_| AppError::InternalError)
}
