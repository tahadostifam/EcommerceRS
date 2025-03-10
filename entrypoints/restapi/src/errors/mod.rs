use serde::Serialize;
pub mod user_errors;

#[derive(Debug, Serialize)]
pub(crate) struct ErrorMessage {
    pub error: String
}

#[derive(Debug, Serialize)]
pub(crate) struct SimpleMessage {
    pub message: String
}

