use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct Admin {
    pub admin_id: u32,
    pub name: Name,
}

#[derive(Debug)]
pub struct Name {
    pub first_name: String,
    pub last_name: String,
}


impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{} {}", self.first_name, self.last_name).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_admin_custom_serialize() {
        let admin = Admin {
            admin_id: 1,
            name: Name {
                first_name: String::from("Alice"),
                last_name: String::from("Smith"),
            },
        };

        let json = serde_json::to_string(&admin).unwrap();
        println!("Admin JSON: {}", json);
    }
}