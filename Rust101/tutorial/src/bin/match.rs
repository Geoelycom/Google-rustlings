// match used for logic
//they must be exhaustive, hence all values must be accounted for in match
// they are similar to if/else statements

fn main() {
  some_int();
  let some_value = true;
  match some_value {
    true => println!("its true"),
    false => println!("its false"),
  }
}

// => this is also called a fat arrow. used in match statements,
// match works in expressions not statement.

// _(underscore) this means everything else
//example 2

fn some_int() {
  let some_number = 3;
  match some_number {
    1 => println!("the number is 1"),
    2 => println!("the number is 2"),
    3 => println!("the number is 3"),
    _ => println!("its something else")
  }
}


//NOTES: prefer match over else .. if when working with a single variable
// match considers all possibilities, giving more robust code
//use underscore(_) to match anything else