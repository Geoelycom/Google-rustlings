// validating References with lifetimes
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. Consider the unsafe program in Listing 10-16, which has an outer scope and an inner scope.


// unsafe program
fn main() {
  let r;
  {
    let x = 5;
    r = &x;
  }
  println!("r: {}", r);
}

// The Borrow Checker Ensures Data Outlives Its References

