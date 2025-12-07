#[test]
fn test_say_hello() {
    let result = hello::say_hello("world");
    assert_eq!(result, "Hello world!");
}
