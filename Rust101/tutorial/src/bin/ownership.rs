// every value in memory has a got a variable holding it. 
// we have the stack and the heap
// the stack contains the pointer, the length and the capacity of the value
// values are ordered, and numbered unlike in a heap where they are unodered


// let s1 = String::from("abc");
// do_stuff(&s1)
// fn do_stuff(s: &String) {
// // code
// }


//exercise

// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
  //inspect_ref(apples);
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });


fn inspect_ref(s: &String) {
  if s.ends_with("s") {
    println!(" the content of this {:?} is plural", s);
  } else {
    println!("{}", s);
  }
  inspect(arg);
}
    

fn change(string: &mut String) {
  if !string.ends_with("s"){
    println!("we are going to add an {} to this", string)
  } else {
    
  }
// 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    //change(&mut arg);
    //println!("I have many {}", arg);

}
    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    //if eat(arg) {
    //    println!("Might be bananas");
    //} else {
    //    println!("Not bananas");
    //}

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    // let mut material = "mud".to_string();
    // println!("This material is just `{}`.", material);
    // bedazzle(&mut material);
    // println!("Wow! Now the material is `{}`!", material);
}

