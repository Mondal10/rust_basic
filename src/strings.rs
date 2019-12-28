// 1. Primitive strings, which are immutable fixed-length string somewhere in memory
// 2. String = Growable, heap-allocated data structure - Use when you need to modify or own string data
// 3. Single quotes for single character eg. 'a'. Double quotes for string eg. "Amit"

pub fn run() {
    let prim_str = "Primitive";
    let heap_str = String::from("Growable String");

    println!("{:?}", (prim_str, heap_str));

    // Get length
    println!("Length: {}", prim_str.len());

    // Pushing
    let mut hello = String::from("Hello");
    hello.push('!'); // push single character
    hello.push_str(" World"); // push string
    println!("{}", hello);

    // Capacity in Bytes
    println!("Capacity: {}", hello.capacity());

    // Check if Empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains the word
    println!("Contains the word world? {}", hello.contains("World"));

    // Replace a work
    println!("Replaced: {}", hello.replace("World", "There"));

    // Loop through the string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len()); // Only throws error when assertion fails, when passed nothing happens
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
