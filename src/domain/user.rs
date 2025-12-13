use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub hobbies: Vec<String>,
    pub phone: Option<String>,
}
