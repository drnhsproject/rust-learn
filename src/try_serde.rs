#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json;
    use crate::domain::user::User;

     #[test]
    fn test_create_json_from_array() {
        let numbers = [1, 2, 3, 4, 5];
        let json = serde_json::to_string(&numbers).unwrap();
        assert_eq!(json, "[1,2,3,4,5]");
        println!("JSON: {}", json);
    }

    #[test]
    fn test_create_user_json() {
        let user = User {
            username: String::from("john doe"),
            email: String::from("hadi@mail.co".to_string()),
            hobbies: vec![String::from("reading"), String::from("coding")],
            phone: None,
        };
        let json = serde_json::to_string(&user).unwrap();
        assert_eq!(json, r#"{"username":"john doe","email":"hadi@mail.co","hobbies":["reading","coding"],"phone":null}"#);
        println!("User JSON: {}", json);
    }

    #[test]
    fn test_create_user_json_not_null() {
        let user = User {
            username: String::from("john doe"),
            email: String::from("hadi@mail.co".to_string()),
            hobbies: vec![String::from("reading"), String::from("coding")],
            phone: Some(String::from("123-456-7890")),
        };
        let json = serde_json::to_string(&user).unwrap();
        assert_eq!(json, r#"{"username":"john doe","email":"hadi@mail.co","hobbies":["reading","coding"],"phone":"123-456-7890"}"#);
        println!("User JSON: {}", json);
    }

    #[test]
    fn test_map_json() {
        let mut values: HashMap<String, i32> = HashMap::new();
        values.insert(String::from("one"), 1);
        values.insert(String::from("two"), 2);
        values.insert(String::from("three"), 3);

        let json = serde_json::to_string(&values).unwrap();
        println!("Map JSON: {}", json);

        let deserialized_map: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
        println!("Deserialized Map: {:?}", deserialized_map);
    }
}