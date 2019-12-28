/**
 * Variables hold primitive data or references to data
 * Variables are immutable by default
 * Rust is block-scoped language
 */

pub fn run() {
    // Variables with `let`
    let name = "Amit Mondal";
    let mut age = 23; // To make the variable mutable use `mut` keyword
    println!("My name is {} and I am {}", name, age); // To avoid warning (age = 23 never used)
    age = 24; // This will throw an error as by default variables are immutable (use mut to make it mutable)
    println!("My name is {} and I am {}", name, age);

    // Variables with `const`
    const ID: i32 = 001; // i32 is used to type the variable as 32bit integer(in const we need to type the variable)
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Amit", 23);
    println!("Name: {} | Age: {}", my_name, my_age);
}
