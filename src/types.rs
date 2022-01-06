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
}
