// The state of asynchrounous Rust

async fn get_some_work_done() {
// The value returned from this function is a "FUTURE"
// for anything to happen, the 'FUTURE' needs to be run on an exector
}

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

















//Task -> Todo(Elyan) -> spawn task for LND connector
//Generate lightning Address
// Parse Address
// Calcuate price and push to management service

fn generate_address(){

}