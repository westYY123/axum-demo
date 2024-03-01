use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RegisterResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct DeleteUserResponse {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct KafkaMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct RedisData {
    pub key: String,
    pub value: String,
}
