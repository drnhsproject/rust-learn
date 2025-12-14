use serde::Serialize;

#[derive(Serialize)]
struct Person {
    first_name: String,
    last_name: String,
    hobbies: Vec<String>,
    addresses: Vec<Address>,
}

#[derive(Serialize)]
struct Data {
    person: Person,
}

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
}

#[cfg(test)]
mod tests {
    use handlebars::handlebars_helper;

    use super::*;

    handlebars_helper!(uppercase: |s: str| s.to_uppercase());

    #[test]
    fn test_serialize_person() {
        let person = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            hobbies: vec![
                "Reading".to_string(),
                "Traveling".to_string(),
                "Cooking".to_string(),
            ],
            addresses: vec![
                Address {
                    street: "123 Main St".to_string(),
                    city: "Anytown".to_string(),
                },
                Address {
                    street: "456 Oak Ave".to_string(),
                    city: "Othertown".to_string(),
                },
            ],
        };

        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_file("hello", "templates/with-hello.mustache")
            .unwrap();

        let data = Data { person };

        let rendered = handlebars.render("hello", &data).unwrap();
        println!("Rendered Template: {}", rendered);
    }

    #[test]
    fn test_each_index() {
        let person = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            hobbies: vec![
                "Reading".to_string(),
                "Traveling".to_string(),
                "Cooking".to_string(),
            ],
            addresses: vec![],
        };

        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_file("person", "templates/person.mustache")
            .unwrap();

        let rendered = handlebars.render("person", &person).unwrap();
        println!("Rendered Template: {}", rendered);
    }

    #[test]
    fn test_each_struct() {
        let person = Person {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            hobbies: vec![
                "Reading".to_string(),
                "Traveling".to_string(),
                "Cooking".to_string(),
            ],
            addresses: vec![
                Address {
                    street: "123 Main St".to_string(),
                    city: "Anytown".to_string(),
                },
                Address {
                    street: "456 Oak Ave".to_string(),
                    city: "Othertown".to_string(),
                },
            ],
        };

        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_file("person", "templates/person.mustache")
            .unwrap();

        let rendered = handlebars.render("person", &person).unwrap();
        println!("Rendered Template: {}", rendered);
    }

    #[test]
    fn test_uppercase_helper() {
        let person = Person {
            first_name: "john".to_string(),
            last_name: "Doe".to_string(),
            hobbies: vec![],
            addresses: vec![],
        };

        let mut handlebars = handlebars::Handlebars::new();
        handlebars
            .register_template_file("uppercase", "templates/uppercase.mustache")
            .unwrap();
        handlebars.register_helper("uppercase", Box::new(uppercase));
        let rendered = handlebars.render("uppercase", &person).unwrap();
        println!("Rendered Template: {}", rendered);
    }
}
