//Topic: Basic arithematic 
//program requirements
//Display the result of the sum of two numbers;
//notes:
// use a function to add two numbers together
fn add_numbers(a: u32, b: u32) -> u32 {
  a + b
}
// use a function to display the results
fn display_result(result: u32) {
//use the "{:?}" macro token in the println to display the result
   println!("{:?}", result);
}
fn main(){
    let result = add_numbers(7,8);
    display_result(result);
}