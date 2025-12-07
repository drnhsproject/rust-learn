fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("you can't use localhost as a host")
    } else {
        println!("Starting application on {}:{}", host, port)
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

mod tests {
    use std::result::Result;

    #[test]
    #[should_panic]
    fn test_start_application() {
        super::start_application("localhost", 8080);
    }

    #[test]
    #[ignore]
    fn test_ignore() {
        println!("this test is ignored")
    }

    #[test]
    fn test_add_again() -> Result<(), String> {
        let result = super::add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err("2 + 2 should be 4".to_string())
        }
    }
}
