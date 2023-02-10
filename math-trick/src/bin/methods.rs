// methods
/*** 
This code defines a struct called Rectangle that has two fields: width and height.
Then we have an implementation block called impl Rectangle in which we defined two methods, area and inc_width which are associated with the Rectangle struct.

The first method fn area(&self) -> u32 calculates the area of the rectangle using its width and height and returns the result. Here &self tells Rust that the method borrows a reference to the Rectangle instance, but doesn't take ownership. The method takes no other arguments.

The second method fn inc_width(&mut self, delta: u32) increments the width of the rectangle by a certain value represented by the variable delta. Here &mut self tells Rust that the method borrows a mutable reference to the Rectangle instance, so it can modify its fields. It takes an argument delta which is a u32 type representing the increment value. It modifies the self.width field by adding the delta value to it.

In the main function, we create an instance of the Rectangle struct and assigns it to a variable rect. And then we use this variable to call the two methods area and inc_width. First, we call area method on the rect variable and get the area of the rectangle and print it.
Then we call the inc_width method on the rect variable and pass the value 5 for delta, this increases the width of the rectangle, and then we again call the area method to get the new area of the rectangle, and print it.

So the code calculates and prints the area of the rectangle, then increases the width of the rectangle and again calculates and prints the new area of the rectangle after the change.

***/

//Examples
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn inc_width(& mut self, delta: u32)  {
    self.width += delta;
  }
}

fn main() {
  entry_point();
  //instantiate a new box and dimensions
  let small_dimensions = Dimension {
    width: 1.0,
    height: 4.0,
    depth: 3.0,
  };

  let small_box = ShippingBox::new(5, BoxColor::Red, small_dimensions);
  small_box.print();
  let mut rect = Rectangle { width: 10, height: 5 };
  println!("old area: {}", rect.area());
  rect.inc_width(5);
  println!("new area: {}", rect.area());
}

 


struct Temperature {
  degrees_f: f64,
}

impl Temperature {
  fn show_temp(&self) {
    println!("{:?} degress F", self.degrees_f);
  }
}

fn entry_point() {
  let hot = Temperature{ degrees_f: 99.9};
  hot.show_temp();
}

//Exercise
enum BoxColor {
  Brown,
  Red,
}

impl BoxColor {
  fn print_color(&self) {
    match self {
      BoxColor::Brown => println!("brown"),
      BoxColor::Red => println!("red"),
    }
  }
}

struct Dimension {
  width: f64,
  height: f64,
  depth: f64,
}

impl Dimension {
  fn print(&self) {
    println!("width: {:?}", self.width);
    println!("height: {:?}", self.height);
    println!("depth: {:?}", self.depth);
  }
}

struct ShippingBox {
  dimension: Dimension,
  weight: i32,
  color: BoxColor,
}


impl ShippingBox {
  fn new(weight:i32, color:BoxColor, dimension:Dimension) -> Self {
    Self {
      weight,
      color,
      dimension,
    }
  }
   
  fn print(&self) {
    self.color.print_color();
    self.dimension.print();
    println!("weight: {:?}", self.weight);
  }

  }


