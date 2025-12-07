mod tests {
    #[test]
    fn test_race_condition() {
        static mut COUNTER: i32 = 0;
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = std::thread::spawn(|| unsafe {
                for _ in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            let _ = handler.join().unwrap();
        }

        println!("Counter: {}", unsafe { COUNTER });
    }

    #[test]
    fn test_atomic() {
        static COUNTER: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
        let mut handlers = vec![];

        for _ in 0..10 {
            let handler = std::thread::spawn(move || {
                for _ in 0..1000000 {
                    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            let _ = handler.join().unwrap();
        }

        println!(
            "Counter: {}",
            COUNTER.load(std::sync::atomic::Ordering::Relaxed)
        );
    }

    #[test]
    fn test_atomic_reference() {
        use std::sync::{Arc, atomic::AtomicI32, atomic::Ordering};

        let counter: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = std::thread::spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            let _ = handler.join().unwrap();
        }

        println!("Counter: {}", counter.load(Ordering::Relaxed));
    }
}
