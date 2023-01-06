
// Array assignment and access
fn main() {
  let mut a: [i8; 10] = [42; 10];
  a[5] = 0;
  println!("a: {:?}", a);

  new_turple()
}

/*
let mut a: [i8; 10] = [42; 10]: This line declares a new mutable array named a and initializes it with 10 elements, each with the value 42.
The type of a is [i8; 10], which means it is an array of i8 values with a length of 10. The mut keyword makes a a mutable binding, meaning that its value can be modified.

a[5] = 0: This line modifies the sixth element of the array (indexes in Rust start at 0) to have the value 0.

println!("a: {:?}", a): This line uses the println! macro to print the value of a to the console. The {:?} syntax is a "debug" placeholder, which tells the println! macro to print the value of a in a debug representation
*/

/**** EXAMPLE 2 ON TURPLE AND ACCESS *******/

fn new_turple() {
  let t: (i8, bool) = (7, true);
  println!("1st index: {}", t.0);
  println!("2nd index: {}", t.1);
}

/**** 
let t: (i8, bool) = (7, true): This line declares a new variable t and initializes it with a tuple containing an i8 value and a bool value. The type of t is (i8, bool), which means it is a tuple with two elements: an i8 value and a bool value.

println!("1st index: {}", t.0): This line uses the println! macro to print the first element of the tuple (which is an i8 value) to the console. The .0 syntax is used to access the first element of the tuple.

println!("2nd index: {}", t.1): This line uses the println! macro to print the second element of the tuple (which is a bool value) to the console. The .1 syntax is used to access the second element of the tuple.

*******/
