
#[tokio::main]
async fn main() {
    do_something().await
}

async fn do_something() {
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("hello world from tokio")
    // do something returns a future.
    // to use it in main function we call await to it.
}