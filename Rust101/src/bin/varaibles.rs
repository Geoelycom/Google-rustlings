// Variables holds primitive data or references to data
// Varaibles are immutable by default
//Rust is a block-scope language.



fn takes_u32(x:u32) {
  println!("u32: {x}");
}

fn takes_i8(y:i8) {
  println!("i8: {y}");
}

fn main() {
  let x = 10;
  let y = 20;

  takes_u32(x);
  takes_i8(y);
}