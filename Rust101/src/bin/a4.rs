// Topic: flow control using if ..else if .. else ..
// program requirements:
// * display > 5, < 5, or = 5 based on the value of a variable
// is > 5, < 5, or == 5, respectively

// notes:
// use a varaible set to any integer value
//use an if.. else if.. else block to determine which message to display
// use the println macro to display the message onto the terminal

//solution
fn main() {
let variable = 10;
if variable > 5 {
  println!(" >5")
} else if variable < 5 {
  println!("<5")
} else {
  println!("=5")
}
}