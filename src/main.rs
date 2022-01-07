// By default variables are immutable
fn main() {
    // mut -> mutable
    let mut x = 5;
    // println! -> macro
    // macro ends with !
    // macros are functions that write more code
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let a: [i32; 10] = [0; 10];
    println!("The array is {:?}", a);

    let mut array = [2];
    println!("The array is {}", array[0]);

    array[0] = 3;
    println!("The updated array is {}", array[0]);
}
