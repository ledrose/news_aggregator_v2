
use axum::{http::StatusCode, response::IntoResponse, Json};
use deadpool_diesel::InteractError;
// use thiserror::Error;
use derive_more::{Display, Error};
use serde_json::json;

pub trait ConvertToApiError {
    fn convert(self) -> ApiError;
}

#[derive(Debug,Display, Error)]
pub enum ApiError {
    #[display(fmt="Invalid login or password")]
    LoginError,
    #[display(fmt="This user already exists")]
    RegistrationError,
    #[display(fmt="Internal error")]
    InternalError,
    #[display(fmt="Not logged into account")]
    NotLoggedError,
    #[display(fmt="Database interaction error")]
    InteractError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"message":format!("{}",self)}))).into_response()
    }
}


impl From<InteractError> for ApiError {
    fn from(value: InteractError) -> Self {
        Self::InteractError
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalError
    }
}