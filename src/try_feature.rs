use std::{thread, time::Duration};

async fn get_async_data() -> String {
    thread::sleep(Duration::from_secs(2));
    return "Hello from async".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_async() {
        let start = Instant::now();
        let result = get_async_data().await;
        let duration = start.elapsed();
        println!("Result: {}", result);
        println!("Duration: {}", duration.as_secs());
    }
}
