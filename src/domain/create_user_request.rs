use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressRequest {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub address: AddressRequest,
}

#[cfg(test)]
mod tests {
    use super::{AddressRequest, CreateUserRequest};
    use serde_json;

    #[test]
    fn test_serialization() {
        let address = AddressRequest {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "CA".to_string(),
            zip_code: "12345".to_string(),
        };

        let create_user_request = CreateUserRequest {
            username: "new_user".to_string(),
            email: "hadi@mail.co".to_string(),
            password: "strong_password".to_string(),
            address,
        };

        let serialized = serde_json::to_string(&create_user_request).unwrap();
        let expected = r#"{"username":"new_user","email":"hadi@mail.co","password":"strong_password","address":{"street":"123 Main St","city":"Anytown","state":"CA","zip_code":"12345"}}"#;
        println!("Serialized: {}", serialized);
        assert_eq!(serialized, expected);

        let deserialized: CreateUserRequest = serde_json::from_str(&serialized).unwrap();
        println!("Deserialized: {:?}", deserialized);
        assert_eq!(deserialized.username, create_user_request.username);
        assert_eq!(deserialized.email, create_user_request.email);
        assert_eq!(deserialized.password, create_user_request.password);
        assert_eq!(deserialized.address.street, create_user_request.address.street);
        assert_eq!(deserialized.address.city, create_user_request.address.city);
        assert_eq!(deserialized.address.state, create_user_request.address.state);
        assert_eq!(deserialized.address.zip_code, create_user_request.address.zip_code);
    }
}