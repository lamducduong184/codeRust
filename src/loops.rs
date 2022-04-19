pub fn run() {
    let mut count = 0;

    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop
    while count <= 100 {
        count += 1;
        if count % 15 == 0 {
            println!("fizzbuzz");

        } else {
            println!("{}", count);
        }
    }
}