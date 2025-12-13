use validator::Validate;
use crate::domain::database_context::DatabaseContext;

#[derive(Debug, Validate)]
#[validate(context=DatabaseContext, 
    schema(
    function = "crate::config::custom_validation::hd::validator::password_equals_confirm_password", 
    skip_on_field_errors = false,
    code = "password_check",
    message = "not match"
),
schema(
    function = "crate::config::context_validator::fw::validator::can_register",
    skip_on_field_errors = true,
    code = "username",
    use_context
)
)]
pub struct Register {
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    pub confirm_password: String,
}

#[cfg(test)]
mod tests {
    use validator::{ValidateArgs, ValidationErrors};

    use super::*;

    #[test]
    fn test_register_validation() {
        let register = Register {
            username: "testuser".to_string(),
            password: "password123".to_string(),
            confirm_password: "password123".to_string(),
        };
        let result = register
        .validate_with_args(&DatabaseContext { total: 5, max_data: 10 });
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_validation_fail() {
        let register = Register {
            username: "testuser".to_string(),
            password: "password".to_string(),
            confirm_password: "password123".to_string(),
        };
        let errors: ValidationErrors = register
        .validate_with_args(&DatabaseContext {total: 5, max_data: 5}).unwrap_err();
        println!("Validation errors: {:?}", errors);
    }
}