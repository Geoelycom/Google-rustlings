// Modules
/**   Module
 Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module 
 // CRATES
 A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you run rustc rather than cargo and pass a single source code file (as we did all the way back in the “Writing and Running a Rust Program” section of Chapter 1), the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.

 Two types of crates
 1. Binary crates( Must have main and they are  executables that we can run.9e.g, commandline program or server )
 2. library crates(does not have main, they just defined functions to be shared and run by another program)

 A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

Example module
 backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

 */

//example of a module. structuring a resturant
mod front_of_house {
  mod hosting {
      fn add_to_waitlist() {}

      fn seat_at_table() {}
  }

  mod serving {
      fn take_order() {}

      fn serve_order() {}

      fn take_payment() {}
  }
}
