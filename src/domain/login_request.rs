use validator::Validate;

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(email, length(min = 3, max = 20, message = "email is required"))]
    username: String,
    #[validate(length(min = 6, max = 20, message = "password min length is 6"))]
    password: String,
    #[validate(nested)]
    name: Name,
}

#[derive(Debug, Validate)]
struct Name {
    #[validate(length(min = 1, message = "first name is required"))]
    first: String,
    #[validate(length(min = 1, message = "last name is required"))]
    last: String,
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use super::LoginRequest;

    #[test]
    fn test_login_request_validation_success() {
        let request = LoginRequest {
            username: "aku@mail.co".to_string(),
            password: "password".to_string(),
            name: super::Name {
                first: "Aku".to_string(),
                last: "Mail".to_string(),
            },
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_login_request_validation_fail() {
        let request = LoginRequest {
            username: "a".to_string(),
            password: "pass".to_string(),
            name: super::Name {
                first: "".to_string(),
                last: "".to_string(),
            },
        };
        assert!(request.validate().is_err());
        let errors = request.validate().unwrap_err();
        println!("Validation errors: {:?}", errors);
    }
}