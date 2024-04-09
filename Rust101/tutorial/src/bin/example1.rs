fn main() {
  let mut x: i8 = 6;
  println!("{x}");
   // print!("the value of x is {}", x);
  while x != 1 {
      if x % 2 == 0 {
         x = x / 2;
      } else {
        x = 3 * x + 1;
      }
      println!(" -> {x}")
  }
  println!();
}


/*

The code enters the loop and checks whether x is even. Since x is currently 6, which is even, x is divided by 2 and the result, 3, is assigned back to x.
The code prints the new value of x, which is 3, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 3, which is odd, x is multiplied by 3 and 1 is added to the result, which is 10. This new value is then assigned back to x.
The code prints the new value of x, which is 10, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 10, which is even, x is divided by 2 and the result, 5, is assigned back to x.
The code prints the new value of x, which is 5, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 5, which is odd, x is multiplied by 3 and 1 is added to the result, which is 16. This new value is then assigned back to x.
The code prints the new value of x, which is 16, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 16, which is even, x is divided by 2 and the result, 8, is assigned back to x.
The code prints the new value of x, which is 8, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 8, which is even, x is divided by 2 and the result, 4, is assigned back to x.
The code prints the new value of x, which is 4, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 4, which is even, x is divided by 2 and the result, 2, is assigned back to x.
The code prints the new value of x, which is 2, to the console using the println! macro.
The code enters the loop again and checks whether x is even. Since x is currently 2, which is even, x is divided by 2 and the result, 1, is assigned back to x.
The code prints the new value of x, which is 1, to the console using the println! macro.
Since x is now equal to 1, the loop condition is no longer satisfied and the loop terminates. The code then prints an empty line to the console using the println! macro
*/