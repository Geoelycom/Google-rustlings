// A slice gives you a view into a larger collection
fn main() {
  let a: [i32; 6] = [10, 20, 30, 40, 50, 60];  // Create an array `a` with six elements
  println!("a: {:?}", a);  // Print the whole array

  let s: &[i32] = &a[2..4];  // Create a slice `s` that refers to elements 2 and 3 of `a`
  println!("s: {:?}", s);  // Print the slice
  ex()
}

/******
a slice is a data type that represents a contiguous section of an array. It is denoted with the syntax [T], where T is the type of the elements in the slice.

Slices are a useful way to work with arrays because they allow you to reference a subarray without copying its elements. This can be more efficient than creating a new array and copying the elements over, especially for large arrays.


Slices have several useful properties and methods that make them a powerful tool for working with arrays in Rust. For example, you can use the len method to get the length of a slice, the is_empty method to check if a slice is empty, and the first and last methods to get the first and last elements of a slice, respectively.

Here is an example that demonstrates some of these methods:


****/

fn ex() {
  let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
  let s: &[i32] = &a[1..5];  // Create a slice `s` that refers to elements 1 through 4 of `a`

  println!("s has {} elements", s.len());  // Print the length of the slice
}