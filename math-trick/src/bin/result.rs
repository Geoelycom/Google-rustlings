/* 
Result
This is a data type that contains one of two types of data
"Successful" data
"Error"data
this is used in serious scenarios where an action needs to be performed, but has the possibility of a failure
examples include, copying a file, connecting to a website or remote server

*/

// Defination
// enum Result <T, E> {
//   ok(T),
//   Err(E)
// }

// example 1
#[derive(Debug)]
enum MenuChoice {
  Mainmenu,
  Start,
  Quit,
}

fn get_menu_choice (input: &str) -> Result<MenuChoice, String> {
  match input {
    "Mainmenu" => Ok(MenuChoice::Mainmenu),
    "Start" => Ok(MenuChoice::Start),
    "Quit" => Ok(MenuChoice::Quit),
    _=> Err("the menu choice not found".to_owned()),
  }
}

fn print_choice(choice: &MenuChoice) {
  println!("choice = {:?}", choice);
}

fn main() {
  let choice: Result<MenuChoice, _> = get_menu_choice("Mainmenu");
  match choice {
    Ok(main_choice) => print_choice(&main_choice),
    Err(e) => println!("error = {:?}", e),
  }
  //println!("choice = {:?}", choice);
}