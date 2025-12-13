use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct UserLoginRequest {
    pub username: String,
    #[serde(rename = "kata_sandi")]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::UserLoginRequest;
    use serde_json;

    #[test]
    fn test_serialization() {
        let login_request = UserLoginRequest {
            username: "test_user".to_string(),
            password: "secure_password".to_string(),
        };

        let serialized = serde_json::to_string(&login_request).unwrap();
        let expected = r#"{"username":"test_user","password":"secure_password"}"#;
        println!("Serialized: {}", serialized);
        assert_eq!(serialized, expected);

        let deserialized: UserLoginRequest = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized: {:?}", deserialized);
        assert_eq!(deserialized.username, login_request.username);
        assert_eq!(deserialized.password, login_request.password);
    }

     #[test]
    fn test_serde_container_attr() {
        let login_request = UserLoginRequest {
            username: "test_user".to_string(),
            password: "secure_password".to_string(),
        };

        let serialized = serde_json::to_string(&login_request).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: UserLoginRequest = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized: {:?}", deserialized);
    }
}