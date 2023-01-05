//Topic: flow control using if, else
//program requirements
//Displays a message based on the value of a boolean variable
// when the variable is set to true, display hello,
// when the vairable is set to false, display goodbye

//notes: use a variable set to either true or false 
// use an if..else block to determine which message to display
// use the println macro to display the messages to the terminal

//solution

fn main() {
  let my_bool = true;
  if my_bool == true {
    println!("hello")
  } else {
    println!("googbye")
  }
}