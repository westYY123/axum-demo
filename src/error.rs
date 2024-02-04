use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

pub type AppResult<T> = Result<T, AppError>;
#[derive(Debug)]
pub enum AppError {
    MissingCredentials,
    WrongCombinationCredentials,
    InternalError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::MissingCredentials => Response::builder()
                .status(403)
                .body(Body::from("Please provide email and password"))
                .unwrap(),
            AppError::WrongCombinationCredentials => Response::builder()
                .status(401)
                .body(Body::from(
                    "please check your combination of email and password",
                ))
                .unwrap(),
            AppError::InternalError => Response::builder()
                .status(500)
                .body(Body::from("There is an internal error"))
                .unwrap(),
        }
    }
}
