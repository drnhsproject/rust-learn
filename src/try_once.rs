use std::sync::Once;

static mut TOTAL_COUNTER: i32 = 0;
static TOTAL_INIT: Once = Once::new();

fn get_total() -> i32 {
    unsafe {
        TOTAL_INIT.call_once(|| {
            TOTAL_COUNTER += 1;
        });
        TOTAL_COUNTER
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_once() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = std::thread::spawn(|| {
                let total = super::get_total();
                println!("Total: {}", total);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }
}
