use super::ErrorMessage;
use actix_web::{HttpResponse, ResponseError, http::header::ContentType};
use derive_more::derive::{Display, Error};
use ecommercers::core::models::product::ProductError;

#[derive(Debug, Display, Error)]
pub enum HttpProductError {
    #[display("internal error")]
    InternalError,

    #[display("product not found")]
    NotFound,

    #[display("invalid data")]
    InvalidData,

    #[display("invalid category")]
    InvalidCategory,

    #[display("permission denied")]
    PermissionDenied,
}

impl From<ProductError> for HttpProductError {
    fn from(value: ProductError) -> Self {
        match value {
            ProductError::InternalError => HttpProductError::InternalError,
            ProductError::NotFound => HttpProductError::NotFound,
            ProductError::InvalidData => HttpProductError::InvalidData,
            ProductError::PermissionDenied => HttpProductError::PermissionDenied,
            ProductError::InvalidCategory => HttpProductError::InvalidCategory,
        }
    }
}

impl ResponseError for HttpProductError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HttpProductError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HttpProductError::NotFound => actix_web::http::StatusCode::BAD_REQUEST,
            HttpProductError::InvalidData => actix_web::http::StatusCode::BAD_REQUEST,
            HttpProductError::InvalidCategory => actix_web::http::StatusCode::BAD_REQUEST,
            HttpProductError::PermissionDenied => actix_web::http::StatusCode::UNAUTHORIZED,
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
