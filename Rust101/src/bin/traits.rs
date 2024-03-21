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


#[async_trait::async_trait]
impl Worker for TransactionTrackerWorker {
    const JOB_NAME: &'static str = "track.address";

    async fn process<B: IIndexedBlockSourceData + Send + Sync + 'static>(
        &self,
        job: Job,
        mut sender: UnboundedSender<ServerMessage>,
        indexed_blockchain_client: B,
    ) -> Result<(), WorkerError> {
        let address_info = serde_json::from_value::<AddressInfo>(job.clone().params)?;

        let (notify_tx, mut notify_rx) = tokio::sync::mpsc::channel(self.config.channel_size); // make unbounded?

        let tx_tracker_config = tx_tracker::TransactionTrackerConfig::new(
            &self.config.electrum_server_url,
            &self.config.bitcoin_network,
        )?;

        tokio::spawn(tx_tracker::run(
            tx_tracker_config,
            address_info,
            notify_tx,
            indexed_blockchain_client,
        ));

        while let Some(tx) = notify_rx.recv().await {
            // Process notification and construct server message
            // Send server message to ZMQ?

            let server_msg = match tx {
                TransactionType::Mempooled(tx) => {
                    ServerMessage::Update(job.clone(), TxStatus::Mempooled, 0, tx.txid())
                }
                TransactionType::Mined(tx, conf) => {
                    ServerMessage::Update(job.clone(), TxStatus::Mined, conf, tx.txid())
                }
                TransactionType::Confirmed(tx, conf) => {
                    ServerMessage::Completed(job.clone(), TxStatus::Confirmed, conf, tx.txid())
                }
                TransactionType::Failed(reason, tx) => {
                    let txid = tx.map(|trans| trans.txid());
                    ServerMessage::Failed(job.clone(), reason, txid)
                }
            };

            // Send message to server
            let send = sender.send(server_msg.clone()).await;

            if send.is_err() {
                println!(
                    "Failed to send server message: {:?} for job: {:?}",
                    server_msg, job
                );
            }
        }

        Ok(())
    }
}



 // unsubscribe
 let unsub_res =
 guard.script_unsubscribe(address.script_pubkey().as_script())?;
if unsub_res {
 let _delay = tokio::time::sleep(Duration::from_millis(
     self.config.scripthash_subscription_delay,
 ))
 .await;
 subscription_attempt += 1;
 continue;
}


// subscribe function

async fn subscribe(
  &self,
  addr_info: &AddressInfo,
) -> Result<electrum_client::Hex32Bytes, FulcrumClientError> {
  // print statement for debugging
  println!("Starting subscripton for address: {}", addr_info.address);
  let mut subscription_attempt = 0;
  let address = Address::from_str(&addr_info.address)?.require_network(self.network)?;

  let start_time = std::time::Instant::now();
  // let us find out if this function makes a network call to subscribe
  loop {
      let guard = self.client.lock().await;
      let sub_status_res = guard.script_subscribe(address.script_pubkey().as_script());

      // println!("{:?}", sub_status_res); (this gives an okay Ok(None))// change here

      match sub_status_res {
          Ok(status) => {
             //  println!("this is another point of test {:?}", status); // change here (this gets printed out)

              if let Some(status) = status {
                  // add another println here
                  let duration = start_time.elapsed();
                  println!("Subscription successful for {} after {} attempts, taking {:?}", addr_info.address, subscription_attempt + 1, duration);
                  break Ok(status);
              } else {
                  if subscription_attempt == self.config.retry_count {
                      // another debug statement
                      let duration = start_time.elapsed();
                      println!("Subscription attempt limit reached for {}, taking {:?}", addr_info.address, duration);
                      let err = FulcrumClientError::ScripthashSubscriptionTimeout(
                          subscription_attempt,
                      );

                      break Err(err);
                  }
                  // Attempt unsubscribe before retrying
                  if let Ok(unsub_res) =  guard.script_unsubscribe(address.script_pubkey().as_script()) {
                      if unsub_res {
                          tokio::time::sleep(Duration::from_millis(self.config.scripthash_subscription_delay)).await;
                          subscription_attempt += 1;
                          continue;
                      } else {
                      print!("Unsubscribe fail for {}", addr_info.address);
                      }
                  } else {
                      println!("Error during unsubscribe for {}", addr_info.address);
                  }

                  // todo(Elyan = 30 minutes(work)): Handle unlikely possibility that unsub_res == false
              }
          }
          Err(err) => {
              println!("Error during subscription for {}: {:?}", addr_info.address, err);
              return Err(FulcrumClientError::ElectrumClient(err));
          }
      }
  }
}
