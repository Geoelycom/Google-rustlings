// Enumeration
// Data can be one of multiple different possiblities or types
//. Each possibility is called a varaint
// Provides information about your program to the compiler
//.. More Robust programs

enum Direction {
  Left, 
  Right,
}

fn main() {
  print_color();
  let go = Direction::Left;
  match go {
     Direction::Left => println!("go left"),
     Direction::Right => println!("go right"),
  }
}

//Custom Enums

enum Color {
  Red,
  Green,
  Yellow,
  Blue,
  Orange
}

fn print_color() {
  let printed_color = Color::Red;
  match printed_color {
    Color::Red => println!("color is red"),
    Color::Green => println!("color is green"),
    Color::Yellow => println!("color is yellow"),
    Color::Blue => println!("color is blue"),
    Color::Orange => println!("color is orange")
  }
}