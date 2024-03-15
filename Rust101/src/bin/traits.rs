// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:

pub fn notify<T: summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}

// this can also be written to take two parameters of same type

pub fn notify2<T: summary>(item1: &T, item2: &T){}
//parameters passed into this must be of thesame type in each item1 and item2


//Implementing a straight.

pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}


use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}


// // Arc mutex
// An Arc<Mutex<T>> in Rust is a way to share ownership of a value of type T across multiple threads, while also providing the ability to mutate the value in a thread-safe manner. Here's a breakdown of what each part means:

// Arc<T>: Arc stands for Atomic Reference Counting. It is a type of smart pointer that provides shared ownership of a value of type T. It's used for sharing data between threads or when a piece of data needs to be accessed in several places within a program and it's not clear when it will be deallocated. The atomic reference counting ensures that the data will not be freed until all references to it have been dropped, making it thread-safe for sharing.

// Mutex<T>: Mutex stands for mutual exclusion. It is a synchronization primitive that can be used to protect shared data from being accessed by multiple threads at the same time. When a thread wants to read or write the shared data, it must first lock the mutex. If the mutex is already locked by another thread, the current thread will block until the mutex becomes available. Once the thread is done with the data, it must unlock the mutex, allowing other threads to access the data. The Mutex wrapper ensures that only one thread can access the data at a time, preventing race conditions.

// Putting them together, Arc<Mutex<T>> allows multiple threads to share ownership of a mutex-protected value and safely access or modify the value across threads. This is especially useful in concurrent programming where you need to both share data between threads and mutate it safely.

// ARC = SHARED OWNERSHIP across threads
// Mutex = thread safety

// two rules to mutex
//1. you must attempt to acquire the lock before you access the data
//2. when you are done accessing the data being guarded by the mutex, you must unlock the data so others can access it after you.

impl FulcrumClient {
  pub const HEADERS_GET_TIP: &'static str = "blockchain.headers.get_tip";
  pub const UNSUBSCRIBE_SCRIPT: &'static str = "blockchain.scripthash.unsubscribe";
  pub const CONFIRMED_BLOCK_HEIGHT: i32 = 6;
  pub const MEMPOOL_HEIGHT: i32 = 0;
}


// suggested fix for the loop for notify

// Assuming this is inside your match height_diff block
height if height >= 6 => {
  // Confirm that we only notify up to the 6th confirmation
  if let Some(value) = send_counter.get(&6) {
      if *value == 0 {
          Self::handle_confirmed_transaction(
              self.client.clone(),
              notify_tx_clone.clone(),
              &tx_history.tx_hash,
              6, // Use exact confirmation number
          )
          .await?;

          send_counter.insert(6, 1);
          println!("Notification for 6th confirmation sent.");
      }
  }
  // After handling the 6th confirmation, ensure no further actions
  break Ok(());
}

//Ensuring blockTip is always correct

// Inside the loop
// let guard = self.client.lock().await;
// let tip_res = guard.raw_call(Self::HEADERS_GET_TIP, vec![])?;
// let blockchain_tip = serde_json::from_value::<BlockTip>(tip_res)?;
// drop(guard);


/// so intstead of having this below
/// let guard = self.client.lock().await;
/// let tip_res = guard.raw_call(Self::HEADERS_GET_TIP, vec![])?;
/// let blockchain_tip = serde_json::from_value::<BlockTip>(tip_res)?;
///drop(guard); after the first mempool = 1. we put it that top of the loop
/// let guard = self.client.lock().await;
/// let tip_res = guard.raw_call(Self::HEADERS_GET_TIP, vec![])?;
/// let blockchain_tip = serde_json::from_value::<BlockchainTip>(tip_res)?;
/// drop(guard)
