// validating References with lifetimes
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. Consider the unsafe program in Listing 10-16, which has an outer scope and an inner scope.


// unsafe program
fn main() {
  let string1 = String::from("long string is long");

  {
      let string2 = String::from("xyz");
      let result = longest(string1.as_str(), string2.as_str());
      println!("The longest string is {}", result);
  }
}

// The Borrow Checker Ensures Data Outlives Its References


fn longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

// rewrite longest function with lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}


struct ImportExcerpt<'a> {
  part: &'a str,
}

fn  main() {
  let novel = String::from("call me ismaila. Some years ago..");
  let first_sentence = novel.split('.').next().expect("could not find a '.' ");
  let i = ImportExcerpt {
    part: first_sentence
  };
}

// understanding lifetimes Elision

