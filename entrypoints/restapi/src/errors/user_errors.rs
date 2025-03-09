use super::ErrorMessage;
use actix_web::{HttpResponse, ResponseError, http::header::ContentType};
use derive_more::derive::{Display, Error};
use ecommercers::core::models::auth::AuthError;

#[derive(Debug, Display, Error)]
pub enum HttpAuthError {
    #[display("internal error")]
    InternalError,

    #[display("invalid payload")]
    InvalidPayload,

    #[display("invalid credentials")]
    InvalidCredentials,

    #[display("token expired")]
    TokenExpired,

    #[display("email already exists")]
    EmailAlreadyExists,
}

impl From<AuthError> for HttpAuthError {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::InternalError => HttpAuthError::InternalError,
            AuthError::InvalidPayload => HttpAuthError::InvalidPayload,
            AuthError::InvalidCredentials => HttpAuthError::InvalidCredentials,
            AuthError::TokenExpired => HttpAuthError::TokenExpired,
            AuthError::EmailAlreadyExists => HttpAuthError::EmailAlreadyExists,
        }
    }
}

impl ResponseError for HttpAuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HttpAuthError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HttpAuthError::InvalidPayload => actix_web::http::StatusCode::BAD_REQUEST,
            HttpAuthError::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            HttpAuthError::TokenExpired => actix_web::http::StatusCode::UNAUTHORIZED,
            HttpAuthError::EmailAlreadyExists => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&ErrorMessage {
                    error: self.to_string(),
                })
                .unwrap(),
            )
    }
}
