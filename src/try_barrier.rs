#[cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};

    #[test]
    fn test_try_barrier() {
        let barrier: Arc<Barrier> = Arc::new(Barrier::new(10));
        let mut handlers = vec![];

        for i in 0..10 {
            let barrier_clone: Arc<Barrier> = Arc::clone(&barrier);
            let handler = std::thread::spawn(move || {
                println!("Join Gammer - {}", i);
                barrier_clone.wait();
                println!("Gammer - {} start!", i);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            let _ = handler.join();
        }
    }
}
