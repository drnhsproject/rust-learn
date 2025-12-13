use validator::Validate;

#[derive(Debug, Validate)]
pub struct CreateCategoryRequest {
    #[validate(custom(function = "crate::config::custom_validation::hd::validator::not_blank"))]
    pub name: String,
    #[validate(length(max = 200))]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::CreateCategoryRequest;
    use validator::{Validate, ValidationErrors};

    #[test]
    fn test_create_category_request_validation() {
        let request = CreateCategoryRequest {
            name: "  ".to_string(),
            description: Some("This is a sample category description".to_string()),
        };
        
        let errors: ValidationErrors = request.validate().unwrap_err();
        println!("Validation errors: {:?}", errors);
    }
}