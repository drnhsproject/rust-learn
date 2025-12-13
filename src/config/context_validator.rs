pub mod fw {
    pub mod validator {
        use crate::domain::{
            database_context::DatabaseContext,
            register::Register
        };
        use std::{borrow::Cow};
        use validator::ValidationError;

        pub fn can_register(
            request: &Register,
            ctx: &DatabaseContext            
        ) -> Result<(), ValidationError> {
            if ctx.total >= ctx.max_data {
                let mut error = ValidationError::new("can_register");
                error.message = Some(Cow::from(
                    format!(
                        "Cannot register user {}. Max data limit of {} reached.", 
                        request.username,
                        ctx.max_data
                    )
                ));
                return Err(error);
            }

            Ok(())
        }
    }
}