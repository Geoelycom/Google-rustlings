#[warn(dead_code)]
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

fn get_array_size(){
  let v = vec![1, 2, 3];
  v[99];
}



fn main() {
  // get_array_size();
  read_username_from_file();
  //access_file();
  //panic!("crash and burn");
}


// Recoverable errors with Result
// the Result enum is defined as having two variants Ok and Err as follows

enum Result<T, E> {
  Ok(T),
  Err(E),
}
//T and E are generic types

fn access_file() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(file_created) => file_created,
        Err(e) => panic!("problem creating the file: {:?}", e),
      },
        other_error => {
          panic!("problem opening the file: {:?}", other_error);
        }
     },
  };
}


//shortcut for Panic on ErrorKind::NotFound
// expect is a good thing to use in error management when writing production grade code. with enough context, each error is communicated for easy inderstanding.

fn read_username_from_file() -> Result<String, io::Error> {  //Result<T,E> 
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}


// propagate error with ?

// fn read_username_from_file() -> Result<String, io::Error> {
//   let mut username_file = File::open("hello.txt")?;

//   let mut username = String::new();

//   username_file.read_to_string(&mut username)?;

//   Ok(username)
// }


//Panic or not to Panic

loop {

  //spin up error message
  let guess: i32 = match guess.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
  };

  if guess < 1 || guess > 100 {
    println!("guess the secret of the number will be between 1 and 100.");
    continue;
  }

  match guess.cmp(&secret_number){

  }
}

// use Enums when you expect the caller to react differently to the possible failure modes available in your library

// Always include a source when wrapping lower-level errors
// Implement std:error:Error for all your error types
//use thiserror to reduce the boilerplate for error writing
//always erase he type of the source error in your code.
