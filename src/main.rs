mod unit_test;

fn main() {
    hello::say_hello("world");
    goodbye::say_goodbye();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_say_hello() {
        assert_eq!(hello::say_hello("world"), "Hello world!");
    }
}
