use serde::{Deserialize, de};

#[derive(Debug,Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub database: DatabaseConfig,
}

#[derive(Debug,Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}