pub fn greet() {
    // Simple print
    println!("Hello from print.rs ::::");

    // Basic formatting
    println!("Display Number: {}", 1);
    println!("{} is from {}", "Amit Mondal", "Mumbai");

    // Positional arguments
    println!(
        "{0} is from {1}. {0} likes to {2}",
        "Amit", "Mumbai", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Amit",
        activity = "Football"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug traits
    println!("{:?}", (10, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
