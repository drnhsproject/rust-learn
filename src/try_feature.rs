use std::{thread, time::Duration};

async fn get_async_data() -> String {
    thread::sleep(Duration::from_secs(2));
    return "Hello from async".to_string();
}

async fn get_database_data(wait: u64) -> String {
    println!("{:?} : get database data", thread::current().id());
    tokio::time::sleep(Duration::from_secs(wait)).await;
    println!("{:?} : hello from database", thread::current().id());
    return "Hello from database".to_string();
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

    #[tokio::test]
    async fn test_database() {
        let mut handlers = vec![];

        for i in 0..5 {
            let handler = tokio::spawn(get_database_data(i));
            handlers.push(handler);
        }

        for handler in handlers {
            let result = handler.await.unwrap();
            println!("Result: {}", result);
        }
    }
}
