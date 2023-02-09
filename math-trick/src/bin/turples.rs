//Turples
/**
A turple is a type of record
it stores data anonymously
there is no need to name fields
useful to return pairs of data from functions
can be "destructured" easily into vairables

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

**/

//METHOD QUICK OVERVIEW
// methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapter 6 and Chapter 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.
//EXAMPLE

enum Access {
  Full,
}

fn one_two_three () -> (i32, i32, i32) { // Turples 
   (1, 2, 3)
}

let numbers = one_two_three();
let (x, y, z) = one_two_three();
println!("{:?}, {:?}", x, numbers.0);
println!("{:?}}, {:?", y, numbers.1);
println!("{:?}, {:?}", z, numbers.2);

let (employee, access) = ("jake", Access::Full);

fn main () {
  cat_coord();
  let coord = (2, 3);
  println!("{:?}, {:?}", coord.0, coord.1);

  let (x, y) = (2, 3);
  println!("{:?}, {:?}", x,y);

  let (name, age) = ("emma", 20);
}

// Exercise Turples

fn cat_coord() -> (i32, i32) {
 let tup = (5, 7);
 let (i, j) = tup;
  if i > 5 {
    println!("{:?} is greater than 5", i);
  } else if i < 5 {
   println!("i is less than 5");
  } else {
    println!("is is equal to 5");
  }
  return tup;
}