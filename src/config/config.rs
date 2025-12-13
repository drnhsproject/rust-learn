use std::env::set_var;
use config::{Case, Config, Environment, File};

use crate::domain::config::AppConfig;

fn config_env() {
    unsafe {
        set_var("DB_HOST", "localhost");
        set_var("DB_PORT", "5432");
        set_var("DB_USER", "admin");
        set_var("DB_PASSWORD", "password");
    }

    let config = Config::builder()
        .add_source(Environment::default().convert_case(Case::Snake))
        .build()
        .unwrap();

    assert_eq!(config.get_string("DB_HOST").unwrap(), "localhost");
    assert_eq!(config.get_int("DB_PORT").unwrap(), 5432);
    assert_eq!(config.get_string("DB_USER").unwrap(), "admin");
    assert_eq!(config.get_string("DB_PASSWORD").unwrap(), "password");
}

fn config_json() {
    let config = Config::builder()
        .add_source(File::new("src/config/application.json", config::FileFormat::Json))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "Rust");
    assert_eq!(config.get_string("version").unwrap(), "1.0.0");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
}

fn config_yaml() {
    let config = Config::builder()
        .add_source(File::new(
            "src/config/application.yaml", 
            config::FileFormat::Yaml
        ))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "Rust App");
    assert_eq!(config.get_int("database.port").unwrap(), 5432);
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");
}

fn config_yaml_deserialize() {
    let config = Config::builder()
        .add_source(File::new(
            "src/config/application.yaml", 
            config::FileFormat::Yaml
        ))
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    assert_eq!(app_config.name, "Rust App");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.host, "localhost");
}
#[cfg(test)]
mod tests {
    use ::config::Config;
    use super::*;

    #[test]
    fn test_load_config() {
        let config = Config::builder().build().unwrap();
        assert!(config.get_string("APP_NAME").is_err())
    }

    #[test]
    fn test_load_config_from_env() {
        config_env();
    }

    #[test]
    fn test_load_config_from_json() {
        config_json();
    }

    #[test]
    fn test_load_config_from_yaml() {
        config_yaml();
    }

    #[test]
    fn test_load_config_from_yaml_deserialize() {
        config_yaml_deserialize();
    } 
}