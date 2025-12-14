use serde::Serialize;

#[derive(Serialize)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
}

#[derive(Serialize)]
pub struct Data {
    pub blog: Blog,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blog_serialization() {
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.register_template_file("blog", "templates/blog.mustache").unwrap();

        // let blog = Blog {
        //     id: 1,
        //     title: "My First Blog".to_string(),   
        //     content: "This is the content of my first blog.".to_string(),
        //     author: Some("Ahmad".to_string()
        // ),
        // };

        // let data = Data { blog };

        let data = serde_json::json!({
            "title": "My First Blog",
            "content": "This is the content of my first blog.",
            "author": "Ahmad"
        });

        let rendered = handlebars.render("blog", &data).unwrap();
        println!("Rendered Blog:\n{}", rendered);
    }
}