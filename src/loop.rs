fn loop() {
    let mut a = 2;

    loop {
        a = a*2;
        if a > 5000 {
            break;
        }

        println!("The num a is {}", a);
    }

    println!("Outside the loop.");
}