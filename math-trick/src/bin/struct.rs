// Structs
// A type that contains muliple pieces of data...
// All or nothing-  cannot have some pieces of data and not others
//.. Each piece of data is called a  "field"
// .. Makes working with data easier
//.. Similar data can be grouped together

struct GroceryItem {
      stock: i32,
      price: f64,
}

//example on structs

enum DrinkFlavour {
  Strawberry,
  Vanilla,
  Chocolate,
  Teal
}

struct FluidOunces {
  flavour: DrinkFlavour,
  fluid_oz: f64,
}

fn print_drink_flavour(drink: FluidOunces) {
  match drink.flavour {
      DrinkFlavour::Strawberry => println!("strawberry"),
      DrinkFlavour::Vanilla => println!("Vanilla"),
      DrinkFlavour::Chocolate => println!("Chocolate"),
      DrinkFlavour::Teal => println!("teal"),
  }
  println!("oz: {:?}", drink.fluid_oz);
}

fn main() {
  let sweet = FluidOunces {
    flavour: DrinkFlavour::Strawberry,
    fluid_oz: 1.0,
  };
  print_drink_flavour(sweet);

  let fruitydrink = FluidOunces {
    flavour: DrinkFlavour::Teal,
    fluid_oz: 6.8,
  };

  print_drink_flavour(fruitydrink);

  let cereal = GroceryItem {
    stock: 10,
    price: 2.99,
  };
  println!("stock: {:?}", cereal.stock);
  println!("price: {:?}", cereal.price);
}