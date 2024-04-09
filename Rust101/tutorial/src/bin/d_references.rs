//Dangling references

fn main () {
  let ref_x: &i32;  // Create a reference to an i32
  {
    let x: i32 = 10;  // Create a stack-allocated variable `x` with the value `10`
    ref_x = &x;  // Assign the reference `ref_x` to `x`
  }  // `x` goes out of scope and is deleted
  println!("ref_x: {ref_x}");  // Attempt to use the dangling reference `ref_x` leads to an error.
}


/** 
a dangling reference is a reference that points to a value that no longer exists. This can happen when a reference is created to a value that is stored in a temporary location, such as a stack-allocated variable, and the value is later deleted or goes out of scope.
In this code, the reference ref_x is created outside of a block, but it is assigned to the stack-allocated variable x inside of a block. When the block ends and x goes out of scope, the value that x points to is deleted, but the reference ref_x is still pointing to it. This creates a dangling reference, because the value that ref_x is pointing to no longer exists.
one solution is to move the println! macro inside the fn before it goes out of scope
**/
