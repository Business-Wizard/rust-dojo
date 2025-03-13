use exercism::hello_world;

#[test]
fn test_hello_should_return_hello_world() {
    let actual = hello_world::hello();
    let expected = "Hello, World!";
    assert_eq!(actual, expected);
}
