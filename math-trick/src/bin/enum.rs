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
  // use crate::Color::Red; //one of this two can be use to pass the enum type to our function
  print_color(Color::Red);
  let go = Direction::Left; //creating an instance of our enum
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

fn print_color(my_color: Color) {
  match my_color {
    Color::Red => println!("color is red"),
    Color::Green => println!("color is green"),
    Color::Yellow => println!("color is yellow"),
    Color::Blue => println!("color is blue"),
    Color::Orange => println!("color is orange")
  }
}

//example 3, ---->>>> rust book

enum IpAddressKind {
  v4,
  v6,
}

//create an instance of IpaddressKind;
let four = IpAddressKind::v4;
let six: IpAddressKind::v6;
// we can now create a function that can take our enum as a parameter. just like we did in case 2
fn route(ip_kind: IpAddressKind) {}
// call the function
route(IpAddressKind::v6);

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin:Quater(state) => println!("state quater from {:?}!", state);

  }
}

// The if let pattern


fn main() {
  let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {}", max);
  }
}

// The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match, where the expression is given to the match and the pattern is its first arm. In this case, the pattern is Some(max), and the max binds to the value inside the Some. We can then use max in the body of the if let block in the same way we used max in the corresponding match arm. The code in the if let block isn’t run if the value doesn’t match the pattern.

