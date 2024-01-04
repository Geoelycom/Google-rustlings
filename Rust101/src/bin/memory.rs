/**
Memory is stored using binary
 bits: 0 or 1
computer optimized for bytes . 1 byte == 8 contiguos bits
fully contiguous 
All data in memory has an "address"
.used to locate data.
.Always the same - only the data changes
..Usually don't utilize address directly.
.Variables handle most of the work
.Items can be located at an address using "offset"
ofset begins at 0
Represents the number of bytes from original address
Normally deal with indexes instead
 */

 // OWNERSHIP FUNDAMENTALS
 /**
  * 
All programs must track their memory usage usage
if they fail to do so, a "leak" occurs
Rust utilizes an "ownership" model to manange memory
the "owner" of memory is responsible for cleaning up the memory
memory can either be "moved" or "borrowed"

  */

  struct Book {
    pages: i32,
    ratings: i32,
  }

  fn display_page_count (book: &Book) { // reference the book from the main fucntion, hence we are borrowing the book from main.
    println!("pages = {:?}", book.pages);
  }

  fn display_ratings(book: &Book) {
    println!("ratings = {:?}", book.ratings);
  }


//exercise
struct Grocery {
  quantity: i32,
  id: i32,
}

fn print_grocery_quantity(grocery: &Grocery) {
  println!("quantity = {:?}", grocery.quantity);
}

fn print_grocery_id(grocery: &Grocery) {
  println!("id = {:?}", grocery.id)
}


fn main() {
  let book = Book {
    pages: 5,
    ratings: 9,
  };

  let grocery = Grocery {
    quantity: 12,
    id: 4,
  };
  print_grocery_quantity(&grocery);
  print_grocery_id(&grocery);
  display_page_count(&book);
  display_ratings(&book);

}