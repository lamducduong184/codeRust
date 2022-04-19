// Vector - Resizable array

use std::collections::btree_map::Values;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);

    // Re-assign values
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    println!("Vector length: {}", numbers.len());
    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;

    }
    println!("Numbers Vec {:?}", numbers);
}