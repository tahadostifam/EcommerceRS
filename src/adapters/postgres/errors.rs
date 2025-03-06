use crate::core::models::product::ProductError;

impl From<diesel::result::Error> for ProductError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ProductError::NotFound,
            diesel::result::Error::InvalidCString(_) => ProductError::InvalidData,
            _ => ProductError::DatabaseError, 
        }
    }
}