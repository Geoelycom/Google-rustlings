/**   
Types of errors: Recoverable errors , 2. unrecoverable errors 
Rust utilizzes the Result<T, E> = recoverable errors and
Panic for unrecoverable errors
Unrecoverable Errors with panic!
Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
[profile.release]
panic = 'abort'
 */

 fn main() {
  // access_level();
  access_file();
 }


//  fn access_level() {
//   let v = vec![1,2,3,5];
//   v[3];
//  }

 /*  
 RECOVERABLE ERRORS WITH RESULTS

 
 */

 use std::fs::File;
 use std::io::ErrorKind;
 
 fn access_file() {
  let greeting_file_result = File::open("hello.txt");

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) =>  match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening file: {:?}", other_error);
      }
  },
 };
}

// Error propagation
/* When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code. */

fn read_username_from_file() -> Result<String, io::Error> {
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