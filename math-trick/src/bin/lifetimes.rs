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

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == 'b' {
      return &s[0..i];
    }

    &s[..]
  }
}

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.