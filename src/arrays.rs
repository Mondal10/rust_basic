// Arrays - Fixed list where elements are the same data types

use std::mem; // Now no need to use std::mem in the below code, just use mem::<whatever method>

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // The array size needs to be 5 and all elements must be of type integer as defined [i32; 5]

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Array can be made mutatable by using `mut` as `let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];`
    // Re-assign value by `numbers[2] = 20;`
    // IMPORTANT: Cannot push or pop elements from the array as the length is fixed.

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); // &numbers is basically referencing the array variable

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
