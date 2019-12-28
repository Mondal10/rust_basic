// Vectors - Resizabel arrays
// We can use push and pop as well

use std::mem; // Now no need to use std::mem in the below code, just use mem::<whatever method>

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assigning value
    numbers[2] = 25;

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); // &numbers is basically referencing the vector variable

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Add/Push element to vector
    numbers.push(10);
    numbers.push(15);

    println!("Added elements in Vector: {:?}", numbers);

    // Remove/Pop last element to vector
    numbers.pop();

    println!("Removed last elements in Vector: {:?}", numbers);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Mutated Vector: {:?}", numbers);
}
