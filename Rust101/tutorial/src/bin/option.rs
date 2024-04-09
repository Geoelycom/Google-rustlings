//working with options data

/*
-- A type that maybe one of two types(Some or None)
-- mainly used in scenarios where data may not be required or is not available
.. unable to find something
.. Ran out of items in a list
.. Form field not filled
*/
// Defination and syntax
enum Option<T> {
  Some(T),
  None
}

// Example

struct Customer {
  age: Option<i32>,
  email: String,
}

fn name_of_customers() {
let mark = Customer {
  age: Some(22), 
  email: "mark@gmail.com".to_owned(),
};

let becky =  Customer {
  age: None, 
  email: "beky@gmail.com".to_owned(),
};

match becky.age{
  Some(age) => println!("Customer is {:?} year old", age),
  None => println!("Customer age is not provided"),
}
}
// example2

struct GroceryItem {
  name: String,
  qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
  let groceries =  vec![
    GroceryItem { name: "bananas".to_owned(), qty: 4, },
    GroceryItem { name: "eggs".to_owned(), qty: 12, },
    GroceryItem { name: "bread".to_owned(), qty: 8, },
  ];

  for item in groceries {
    if item.name == name {
      return Some(item.qty);
    }
  }
}

//Demo
 struct Survey {
  q1: Option<i32>,
  q2: Option<bool>,
  q3: Option<String>,
 }

 fn main() {
  print_student_locker_assignment();
  let response = Survey {
    q1: Some(12),
    q2: Some(true),
    q3: Some("A".to_owned()),
  };

  match response.q1 {
    Some(ans) => println!("q1: {:?}", ans),
    None => println!("q1: no response"),
  }

  match response.q2 {
    Some(ans) => println!("q2: {:?}", ans),
    None => println!("q2: no response"),
  }

  match response.q3 {
    Some(ans) => println!("q3: {:?}", ans),
    None => println!("q3: no response"),
  }
 }


 // activity
 struct StudentDetails {
  name: String,
  locker_assignment: Option<i32>,
 }

 fn print_student_locker_assignment() {
  let student_assigned_locker_1 =  StudentDetails {
    name: "joan".to_owned(),
    locker_assignment: Some(20),
  };

  let student_assigned_locker_2 =  StudentDetails {
    name: "mike".to_owned(),
    locker_assignment: None,
  };
    match student_assigned_locker_1.locker_assignment {
      Some(assignment) => println!("the assigned number is: {:?}", assignment),
      None => println!("there is no number assigned to this locker"),
    }

    match student_assigned_locker_2.locker_assignment {
      Some(assignment) => println!("the assigned number is: {:?}", assignment),
      None => println!("there is no assigned number for this locker")
    }
 }