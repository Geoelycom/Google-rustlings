//let array = [10, 20, 30];
fn main() {
  let array = [10, 20, 30];
  print!("Iterating over array:");
  for n in array {
      print!(" {n}");
  }
  println!();

  print!("Iterating over range:");
  for i in 0..3 {
      print!(" {}", array[i]);
  }
  println!();
}