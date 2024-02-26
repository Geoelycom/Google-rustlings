//DATA STRUCTURES ONE
//VECTORS
/**
 support Multiple pieces of data. Each piece of data must be of thesame type
 used for lists of informations
 can add, remove, and tranverse the entries
A vector allows you to store a variable number of values next to each other.
vectors contain multiple pieces of similar data. 
use for..in loop to interate through items of a vector
 **/
//syntax
fn main() {
  test_scores();
  print_numbers();
  let v: Vec<i32> = Vec::new();
}

fn new_vec() {
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

//types of methods on vecs
struct Test {
  scores: i32,
}

fn test_scores(){
  let my_scores = vec![
    Test { scores: 90 },
    Test { scores: 77 },
    Test { scores: 69 },
    Test { scores: 80 },
    
  ];

  for test in my_scores {
    println!("test scores: {:?}", test.scores)
  }
}

// exercise

fn print_numbers() {
  let my_numbers = vec![10, 20, 30, 40];
  for number in &my_numbers {
    match number {
      30 => println!("thirty"),
      _=> println!("the number is {:?}", number),
    }
  }
  println!("the number of element: {:?}", my_numbers.len());
}

// Todo: update exercise for new hires/students visiting the repo