use serde::Serialize;
pub mod user_errors;

#[derive(Debug, Serialize)]
pub(crate) struct ErrorMessage {
    error: String
}

