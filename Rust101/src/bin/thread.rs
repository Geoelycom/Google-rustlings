
fn get_two_sites() {
  //spawn two threads to do work
  let thread_one = thread::spawn(|| download("https://github.com/easepay"));
  let thread_two = thread::spawn(|| download("https://github.com/geolycom"));

  // wait for both services to complete
  thread_one.join().expect("thead one panicked");
  thread_two.join().expect("thead two panicked");

}


// Using async to aviod wasting time since this process is a simple task


async fn get_two_sites_async () {
  //create two different "futures" which, when run to completions,
  // asynchronously download the webpages

  let future_one_website = download_async("https://github.com/easepay");
  let future_two_website = download_async("https://github.com/geoelycom");
  
  // run both futures to completion at the same time
  join!(future_one_website, future_two_website);
}