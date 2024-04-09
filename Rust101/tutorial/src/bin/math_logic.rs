//Decision exercise with match
fn main() {
  multiple_match();
  let my_string = true;
  match my_string {
     true => println!("Yes my string is true"),
     false => println!("No my string is false"),
  }
}


fn multiple_match(){
  let number = 3;
  match number {
  1 => println!("my interger is 1"),
  2 => println!("my interger is 2"),
  3 => println!("my interger is 3"),
  4 => println!("my interger is 4"),
  _ => println!("I'm a number not listed"),
}
}