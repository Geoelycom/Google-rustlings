fn largest(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

//Generics and how we can reduce code duplication

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);
  assert_eq!(*result, 100);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

  let result = largest(&number_list);
  println!("The largest number is {}", result);
  assert_eq!(*result, 6000);
}


fn largest<T>(list: &[T]) -> &T {}
// Explaining the above line
  // We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.


  // Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.