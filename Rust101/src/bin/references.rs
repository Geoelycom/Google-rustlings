fn main() {
  let mut x: i32 = 10;
  let ref_x: &mut i32 = &mut x;
  *ref_x = 20;
  println!("x: {x}");
}


/**

EXPLANATION
The first line of the code declares a variable x with the type i32 (a signed 32-bit integer) and assigns it the value 10. The let keyword is used to create a new variable, and the mut keyword indicates that the variable is mutable, meaning that its value can be changed.
The &mut syntax indicates that this is a mutable reference, which means that the value being referenced can be modified through the reference.
The third line of the code uses the dereference operator (*) to modify the value being pointed to by the reference. In this case, the value of x is changed from 10 to 20.
The fourth line of the code uses the println! macro to print the value of x to the console. The {x} syntax is used to interpolate the value of x into the string.
Overall, this code creates a mutable variable x with the value 10, creates a mutable reference to x, modifies the value being pointed to by the reference to be 20, and then prints the value of x to the console.
A reference is a way to borrow a value from one place in your code and use it in another place, without taking ownership of the value. This is similar to how variables work in languages like JavaScript, where you can create a variable and assign it a value, and then use that value in different parts of your code.
However, in Rust, references have some additional properties that make them a powerful and safe way to work with values. One key difference is that Rust references are immutable by default, which means that once you create a reference, you cannot change the value that it points to. This helps to prevent unintended side effects and makes it easier to reason about your code.
the reference syntax is the & operator.


**/
