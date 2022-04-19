// Arrays - Fixed list where elements are the same data types

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    print!("{:?}", numbers);

    // Get single val

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}