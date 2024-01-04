// Overloading in Rust === not supported
/** 
Each function has a single implementation:
Always takes a fixed number of parameters.
Always takes a single set of parameter types.
Default values are not supported:
All call sites have the same number of arguments.
Macros are sometimes used as an alternative.
explian this for me

***/

fn pick_one<T>(a: T, b: T) -> T {
  if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
  println!("coin toss: {}", pick_one("heads", "tails"));
  println!("cash prize: {}", pick_one(500, 1000));
}

/** 
in the code above, the pick_one function is defined as a generic function by using the angle brackets <T> in its signature. The T type parameter is used to represent the type of the two parameters "a" and "b" and the return value of the function. When the pick_one function is called, the actual types of "a" and "b" and the return value will be determined by the arguments passed to the function.
**/