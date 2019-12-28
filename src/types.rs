/**
 * Primitive Types:
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Character (char)
 * Tuples
 * Arrays
 */

// Rust is statically typed language, which means it must know the types of all variables at compile time, however, the compiler can ususally infer what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 2453667858;

    // Find max size
    println!("Max size i32: {}", std::i32::MAX);
    println!("Max size i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true; // implicit type
    let is_private: bool = false; // explicit type

    // Get Boolean from expression
    let is_greater = 10 > 5; // we can explicitly set using :bool

    // Character
    let a = "L";
    let face = "\u{1F600}"; // Can contain emoji unicode

    println!(
        "{:?}",
        (x, y, z, is_active, is_private, is_greater, a, face)
    );
}
