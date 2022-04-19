pub fn run() {
    // let mut hello = "Hello";
    // Growable
    let mut hello = String::from("Hello ");
    hello.push('W');
    hello.push_str("orld");
    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop throght string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}