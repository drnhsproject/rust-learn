use std::cell::RefCell;

thread_local! {
    pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
}

mod tests {
    #[test]
    fn test_thread_local_data() {
        let handler = std::thread::spawn(|| {
            super::NAME.with_borrow_mut(|name| {
                *name = "Hadi".to_string();
            });

            super::NAME.with_borrow(|name| {
                println!("name {}", name);
            });
        });

        handler.join().unwrap();

        super::NAME.with_borrow(|name| {
            println!("name {}", name);
        });
    }

    #[test]
    fn test_thread_panics() {
        let handler = std::thread::spawn(|| {
            panic!("thread panicked");
        });

        match handler.join() {
            Ok(_) => {
                panic!("thread should have panicked");
            }
            Err(_) => {
                println!("thread panicked");
            }
        }

        println!("Application is finished");
    }
}
