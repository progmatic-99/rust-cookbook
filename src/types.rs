// Register types
fn main() {
  // i32 -> 32 bit integer
  // Annotate the type of variable
  let x: i32 = 5;

  x.print_type();
  println!("The value of x is {}", x);

  // Type inference
  let y = 4;
  y.print_type();

  // Tuples
  let tuple = (1, "Hello", false);
  println!("The first two values are {} {}", tuple.0, tuple.1);

  // Destructuring
  let (val_one, val_two, _) = tuple;
  println!("The first two values are {} {}", val_one, val_two);

  // Array
  let a:[i32; 10] = [0; 10]
  println!("The array is {:?}", a);
}
