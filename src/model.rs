use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry{
    pub service: String,
    pub username: String,
    pub password: String,
}

impl PasswordEntry{
    pub fn new(service: &str, username: &str, password: &str ) -> Self{
        Self {
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}