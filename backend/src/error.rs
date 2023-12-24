
use actix_web::{error, HttpResponse, http::header::ContentType};
// use thiserror::Error;
use derive_more::{Display, Error};

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
}



impl error::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match &self {
            Self::NotLoggedError|Self::LoginError => actix_web::http::StatusCode::UNAUTHORIZED,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}