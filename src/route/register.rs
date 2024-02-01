use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;

use crate::model::user::{User, UserDto};

pub async fn register(req: Request) -> impl IntoResponse {
    println!("{req:?}");
    let (_, body) = req.into_parts();
    let bytes = body.collect().await;
    match bytes {
        Ok(bytes) => {
            let bytes = bytes.to_bytes();
            let user: Result<User, serde_json::Error> = serde_json::from_slice(&bytes);
            match user {
                Ok(user) => {
                    println!("{user:?}");
                    let user = UserDto::from(user);
                    println!("{user:?}");
                    Response::builder()
                        .status(201)
                        .body(Body::from("'message': 'success'"))
                        .unwrap()
                }
                Err(_) => Response::builder().status(404).body(Body::empty()).unwrap(),
            }
        }

        Err(_) => Response::builder().status(404).body(Body::empty()).unwrap(),
    }
}
