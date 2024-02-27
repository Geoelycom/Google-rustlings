// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:

pub fn notify<T: summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// this can also be written to take two parameters of same type

pub fn notify2<T: summary>(item1: &T, item2: &T){}
//parameters passed into this must be of thesame type in each item1 and item2


//Implementing a straight.

pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}