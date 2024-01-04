//Functions
fn main() {
  fizzbuzz_to(20);   // Defined below, no forward declaration needed
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
      return false;  // Corner case, early return
  }
  lhs % rhs == 0     // The last expression is the return value
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
  match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
      (true,  true)  => println!("fizzbuzz"),
      (true,  false) => println!("fizz"),
      (false, true)  => println!("buzz"),
      (false, false) => println!("{n}"),
  }
}

fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
  for n in 1..=n {
      fizzbuzz(n);
  }
}

/*
The match keyword is used to check different possibilities for the input n. Inside the match statement, the value being matched is a tuple of two boolean values: (is_divisible_by(n, 3), is_divisible_by(n, 5)). The is_divisible_by is a function that check if n is divisible by 3 or 5.

The match statement then has four different cases, or "arms", that correspond to different possibilities for the tuple of booleans.

(true,  true)  => println!("fizzbuzz"),
This arm is run if n is divisible by both 3 and 5, and it will print the string "fizzbuzz"


(true,  false) => println!("fizz"),
This arm is run if n is divisible by 3 but not by 5, and it will print the string "fizz"


(false, true)  => println!("buzz"),
This arm is run if n is divisible by 5 but not by 3, and it will print the string "buzz"


(false, false) => println!("{n}"),
This arm is run if n is not divisible by either 3 or 5, and it will print the value of n.

Basically the match statement check different possibilities of the tuple if its (true, true), (true, false), (false, true) or (false, false) and prints corresponding values based on the tuple value.

**/