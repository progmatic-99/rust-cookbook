fn for_loop() {
    // right hand limit is exclusive
    for i in 0..10 {
        println!("The value of i is {}", i);
    }

    // Inclusive range
    for j in 0..=10 {
        println!("The value of j is {}", j);
    }

    // Iterators
    let arr = [1, 2, 3];

    for a in arr {
        println!(a);
    }
}
