use super::ErrorMessage;
use actix_web::{HttpResponse, ResponseError, http::header::ContentType};
use derive_more::derive::{Display, Error};
use ecommercers::core::models::category::CategoryError;

#[derive(Debug, Display, Error)]
pub enum HttpCategoryError {
    #[display("internal error")]
    InternalError,

    #[display("category not found")]
    NotFound,

    #[display("invalid data")]
    InvalidData,

    #[display("category already exist")]
    CategoryAlreadyExist,
}

impl From<CategoryError> for HttpCategoryError {
    fn from(value: CategoryError) -> Self {
        match value {
            CategoryError::InternalError => HttpCategoryError::InternalError,
            CategoryError::NotFound => HttpCategoryError::NotFound,
            CategoryError::InvalidData => HttpCategoryError::InvalidData,
            CategoryError::CategoryAlreadyExist => HttpCategoryError::CategoryAlreadyExist,
        }
    }
}

impl ResponseError for HttpCategoryError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            HttpCategoryError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            HttpCategoryError::NotFound => actix_web::http::StatusCode::BAD_REQUEST,
            HttpCategoryError::InvalidData => actix_web::http::StatusCode::BAD_REQUEST,
            HttpCategoryError::CategoryAlreadyExist => actix_web::http::StatusCode::BAD_REQUEST,
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
