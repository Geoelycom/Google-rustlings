/* 
HASH MAP
.. This is a collection that stores data as key:value pairs
1. data is located using the "key",
2. the data is the value 
.. Similar to objects in javascript and definations in a dictionary
they are very fast to retrieve data using the key
*/
//Example
// use std::collections::HashMap;

// let mut people = HashMap::new();


// people.insert("susan", 21),
// people.insert("john", 19),
// people.insert("mary", 25),
// people.insert("mike", 11),
// people.insert("sophia", 41),


//understanding rust impl and method

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}