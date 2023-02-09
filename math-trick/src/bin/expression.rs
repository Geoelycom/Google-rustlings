//Expressions
/**
 Rust is an expression-based language
 most things are evaluated nd returned some value
 expressions value coalesce to a single point.
 can be used for nesting logic

 */

 enum Menu {  
   Burger, 
   Fries, 
   Drink,
 }

 let paid = true;
 let item = Menu::Drink;
 let drink_type = "water";
 let order_placed = match item {
     Menu::Drink => {
      if drink_type == "water" {
        true
      } else {
        false
      }
     }
     _ => true,
 };

 enum Access {
  Admin,
  Manager,
  User,
  Guest,
 }

 fn main() {
  //secret file: admins only
  let access_level = Access::Guest;
  let can_access_file = match access_level {
    Access::Admin => true,
    _ => false,
  };
  println!("can access: {:?}", can_access_file)
 }

 // exercise

 fn expression() {
  let bool_variable = 100;
  let its_big = match bool_variable {
     100...200 => println!("its bigger than 100"),
     _ => println!("its smaller than 100")
  };
}
