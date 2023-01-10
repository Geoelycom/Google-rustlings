//strings and str

fn main() {
  let s1: &str = "hello"; //type is set to a reference string slice
  println!("s1: {s1}");

  let mut s2: String = String::from("hello");
  println!("s2: {s2}");
  s2.push_str(s1);
  println!("s2: {s2}");
}

/***  
a variable s1 is declared using the let keyword, and it is assigned the string literal "Hello". The type of the variable is specified as &str, which is a reference to a string slice. A string slice is a reference to a portion of a String and it is denoted by the & symbol. Here s1 is pointing to the memory location of the string literal "Hello".

The next line creates a new variable s2 using the String::from function, which creates a new String instance from a string literal. This variable is then assigned the value "Hello ". The mut keyword is used to indicate that the variable is mutable and its value can be modified.

The following line then calls the push_str method on s2 variable which takes an argument of type &str and appends it to the end of s2 variable. Here s2 variable is appending the value stored at the memory location pointed by s1 variable.

In Rust, the String type is a growable, mutable, owned, UTF-8 encoded string type. It is different from a string literal, which is a string slice (&str) that is stored in the binary of the program and is not growable or mutable.

`String::from` is a static method of the String struct, which means that it is called using the double colon syntax String::from. It takes one argument, which is a string literal, and returns a new String instance that contains the same data as the string literal. The from function will allocated memory to store the string and will copy the content of the string literal to this allocated memory.

This method also validates that the provided string is a valid UTF-8 encoded string and will return an error in case it is not.

For example, the following code creates a new String instance my_string from the string literal "Hello, world!":

***/

let my_string = String::from("Hello, world!");
/** 
In general, when you have a string literal and you want to use it as a String, you will want to use the String::from method to create a new String instance. This allows you to use all the methods provided by the String type, such as push, pop, truncate and more, in addition to passing the string to functions that accept a String argument.

The &str type is a slice of an owned String and it doesn't own the memory it's pointing to. Instead it borrows it and points to the memory location where the string data is stored.
**/