mod mutex_data;
mod race_condition;
mod thread_local_data;
mod unit_test;

use std::thread;

fn main() {
    let current_thread = thread::current();
    println!("Current thread: {:?}", current_thread.name().unwrap());

    hello::say_hello("world");
    goodbye::say_goodbye();
}

#[cfg(test)]
mod tests {
    use std::{thread, thread::JoinHandle, time};

    #[test]
    fn test_say_hello() {
        assert_eq!(hello::say_hello("world"), "Hello world!");
    }

    #[test]
    fn test_thread() {
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Counter {}", i);
                thread::sleep(time::Duration::from_secs(1));
            }
        });

        println!("Application is finished");
        thread::sleep(time::Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter {}", i);
                thread::sleep(time::Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });

        println!("waiting handler");

        let result = handle.join();

        match result {
            Ok(counter) => println!("Total counter: {}", counter),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Application is finished");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current_thread = thread::current();
        for i in 0..=5 {
            match current_thread.name() {
                Some(name) => {
                    println!("{}: Counter {}", name, i)
                }
                None => {
                    println!("{:?}: Counter {}", current_thread.id(), i)
                }
            }
            thread::sleep(time::Duration::from_secs(1));
            counter += 1;
        }

        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Result 1: {}", result1);
        println!("Result 2: {}", result2);
        println!("Total counter: {}", result1 + result2);
        println!("Application is finished");
    }

    #[test]
    fn test_async_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => {
                println!("Total counter 1: {}", counter)
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }

        match result2 {
            Ok(counter) => {
                println!("Total counter 2: {}", counter)
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
        println!("Application is finished");
    }

    #[test]
    fn test_closures() {
        let current_thread = thread::current();
        println!("Current thread: {:?}", current_thread.name().unwrap());

        let name = String::from("world");
        let closure = move || {
            thread::sleep(time::Duration::from_secs(1));
            println!("Hello {}!", name);
        };

        let handle = thread::spawn(closure);
        handle.join().unwrap();

        println!("Application is finished");
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("MyThread".to_string());

        let handler = factory
            .spawn(calculate)
            .expect("failed to create new thread");

        let result = handler.join();

        match result {
            Ok(counter) => {
                println!("Total counter: {}", counter)
            }
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
        println!("Application is finished");
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle = thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(2));
            sender.send("hello from thread".to_string())
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("Received message: {}", message);
        });

        let _ = handle.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_queue_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(time::Duration::from_secs(2));
                sender
                    .send(format!("send from sender thread {}", i))
                    .unwrap();
            }
            let _ = sender.send("exit sender".to_string());
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "exit sender" {
                    break;
                }
                println!("Received message: {}", message);
            }
        });

        let _ = handle.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(time::Duration::from_secs(2));
                sender.send(format!("hello from thread {}", i)).unwrap();
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("Received message: {}", value);
            }
        });

        let _ = handle.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle2 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(time::Duration::from_secs(2));
                sender2
                    .send(format!("hello from thread sender2 {}", i))
                    .unwrap();
            }
        });

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(time::Duration::from_secs(2));
                sender
                    .send(format!("hello from thread sender1 {}", i))
                    .unwrap();
            }
        });

        let handle3 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("Received message: {}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
    }
}
