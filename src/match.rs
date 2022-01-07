fn match_loop() {
    let x = true;
    let y = false;

    match (x, y) {
        (true, true) => println!("x & y are true."),
        (false, false) => println!("x & y are false."),
        (true, false) => println!("x is true & y is false."),
        (false, true) => println!("x is false & y is true."),
    }
}
