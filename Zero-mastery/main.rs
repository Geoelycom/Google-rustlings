use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}


// data Types
// memory only stores binary data
// anything can be represented in a binary format..
// programs determine what the binary represents
// basic types that are universally useful are provided by the language

/* 
Basic data types
Boolean.. true/false
Integer.. 1,2,50,99, -2
Double/float 1.1, 5.5, 200.0001, 2.0
character ..'A', 'B', 'c', 'D', 'E', '$'
String .. "hello", "string", "this is a string", "its 43"
*/

//fundamentals

// anatomy of a  rust functions
/* 
fn = keyword
name.. function name
parameters'' types of data the function will work with(a: i32, b: i32)

debug print = {:?}

*/