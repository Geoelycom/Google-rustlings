//DATA STRUCTURES ONE
//VECTORS
/**
 support Multiple pieces of data. Each piece of data must be of thesame type
 used for lists of informations
 can add, remove, and tranverse the entries
A vector allows you to store a variable number of values next to each other.
 **/
//syntax
fn main() {
  let v: Vec<i32> = Vec::new();
}

fn newVec() {
  let v = vec![1, 2, 3, 4, 5];

  // Using indexing
  let third: &i32 = &v[2];
  println!("The third element is {third}");


  // Using GET which provides us with an Option<&T>
  let third: Option<&i32> = v.get(2);
  match third {
      Some(third) => println!("The third element is {third}"),
      None => println!("There is no third element."),
  }
}
