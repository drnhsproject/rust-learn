pub mod hd{
    pub mod validator {
        use std::borrow::Cow;
        use validator::{ValidationError};
        use crate::domain::register::Register;

        pub fn not_blank(value: &str) -> Result<(), ValidationError> {
            if value.trim().is_empty() {
                let mut error = ValidationError::new("not_blank");
                error.message = Some(Cow::from("value cannot be blank"));
                return Err(error);
            }

            Ok(())
        }

        pub fn password_equals_confirm_password(register: &Register) -> Result<(), ValidationError> {
            if register.password != register.confirm_password {
                let mut error = ValidationError::new("password_mismatch");
                error.message = Some(Cow::from("Password and confirm password do not match"));
                return Err(error);
            }

            Ok(())
        }
    }
}