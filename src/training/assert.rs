pub fn run() {
    let some_string = "Hello";

    assert_eq!(5, some_string.len());

    let other_string = String::with_capacity(5);

    assert_eq!(5, other_string.capacity());

}