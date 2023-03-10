// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:

pub fn notify<T: summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// this can also be written to take two parameters of same type

pub fn notify2<T: summary>(item1: &T, item2: &T){}
//parameters passed into this must be of thesame type in each item1 and item2
