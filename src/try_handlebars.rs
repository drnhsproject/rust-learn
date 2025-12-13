use std::collections::HashMap;

fn handlebars_template() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_string("template1", "Hello {{name}}!")
        .unwrap();
    handlebars
        .register_template_string("template2", "Goodbye {{name}}!")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Alice");
    let result1 = handlebars.render("template1", &data).unwrap();
    let result2 = handlebars.render("template2", &data).unwrap();
    println!("{}", result1); // Outputs: Hello Alice!
    println!("{}", result2); // Outputs: Goodbye Alice!
}

fn handlebars_template_nested() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_string(
            "template1",
            "Hello {{person.first_name}} {{person.last_name}}!",
        )
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();

    person.insert("first_name", "Abdul");
    person.insert("last_name", "Malik");

    data.insert("person", person);

    let result1 = handlebars.render("template1", &data).unwrap();
    println!("{}", result1);
}

fn handlebars_html_escape() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_string(
            "template1",
            "Hello {{{person.first_name}}} {{{person.last_name}}}!",
        )
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();

    person.insert("first_name", "<b>Abdul</b>");
    person.insert("last_name", "<h2>Malik</h2>");

    data.insert("person", person);

    let result1 = handlebars.render("template1", &data).unwrap();
    println!("{}", result1);
}

fn handlebars_template_file() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Ahmad");

    let result1 = handlebars.render("hello", &data).unwrap();
    println!("{}", result1);
}

fn handlebars_template_file_with() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_file(
            "with_hello",
            "templates/with-hello.mustache",
        )
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();

    person.insert("first_name", "Abdul");
    person.insert("last_name", "Maliq");

    data.insert("person", person);

    let result1 = handlebars.render("with_hello", &data).unwrap();
    println!("{}", result1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handlebars() {
        handlebars_template();
    }

    #[test]
    fn test_handlebars_nested() {
        handlebars_template_nested();
    }

    #[test]
    fn test_handlebars_html_escape() {
        handlebars_html_escape();
    }

    #[test]
    fn test_handlebars_template_file() {
        handlebars_template_file();
    }

    #[test]
    fn test_handlebars_template_file_with() {
        handlebars_template_file_with();
    }
}
