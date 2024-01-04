use std::io;

fn main() {
  println!("Enter your weight (kg): ");
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let weight:f32 = input.trim().parse().unwrap();
  dbg!(weight);

  println!("Input: {}", input);

  let mars_weight = calc_mars_weight(weight);
  println!("weight on mars: {}kg", mars_weight);
}

fn calc_mars_weight(weight: f32) -> f32 {
  (weight / 9.81) * 3.711
}


//ownership rules in rust

//1. Each value in Rust is owned by a variable
//2. When the owner goes out of scope. the value will be deallocated.
//3. there can only be one owner at a time