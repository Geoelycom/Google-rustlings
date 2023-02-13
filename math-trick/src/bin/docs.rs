// Documentation for programs
// we use three /// for cargo documentation 
//example
/**
 to generate cargo documentation for our program, we used the cargo command
 `cargo doc --open( this automatically open our documentations)
 */

 /* 
 Standard Library

 
 */

 fn main() {
  transform_string();
  // let numbers = vec![1, 2, 3, 4, 5, 6, 7]
 }

 fn transform_string() -> String {
  let word = String::from("hello world");
  let transform_string = &word.to_uppercase();
  println!("{:?}", transform_string);
  return transform_string.to_string();
 }