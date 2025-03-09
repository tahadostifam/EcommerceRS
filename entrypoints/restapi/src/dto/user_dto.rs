use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserRegisterDTO {
    pub name: String, 
    pub email: String, 
    pub password: String, 
}