use std::{sync::Arc, time::Duration};

use axum::{extract::State, Json};
use rdkafka::{
    message::{Header, OwnedHeaders},
    producer::FutureRecord,
};

use crate::error::{AppError, AppResult};

use super::{idl::KafkaMessage, AppData};

pub async fn send_message(
    State(app_data): State<Arc<AppData>>,
    Json(req): Json<KafkaMessage>,
) -> AppResult<()> {
    app_data
        .kafka_client
        .send(
            FutureRecord::to("quickstart-events")
                .payload(&format!("Message {}", req.message))
                .key(&format!("Key {}", req.message))
                .headers(OwnedHeaders::new().insert(Header {
                    key: "header_key",
                    value: Some("header_value"),
                })),
            Duration::from_secs(0),
        )
        .await
        .map_err(|(_, _)| AppError::KafkaSendFailed)?;
    Ok(())
}
