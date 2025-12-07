mod tests {
    #[test]
    fn test_mutex_data() {
        use std::sync::{Arc, Mutex, MutexGuard};

        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = std::thread::spawn(move || {
                for _ in 0..1000000 {
                    let mut data: MutexGuard<i32> = counter_clone.lock().unwrap();
                    *data += 1;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            let _ = handler.join().unwrap();
        }

        println!("Counter: {}", *counter.lock().unwrap());
    }
}
