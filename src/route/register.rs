use axum::Json;

use serde::{Deserialize, Serialize};

use crate::{
    error::AppResult,
    model::user::{self, insert_user},
};
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(Json(data): Json<RegisterRequest>) -> AppResult<String> {
    println!("{data:?}");
    let user = insert_user(user::ActiveModel {
        id: Default::default(),
        username: sea_orm::ActiveValue::Set(data.username),
        password: sea_orm::ActiveValue::Set(data.password),
    })
    .await?;
    Ok(user.id.to_string())
}
