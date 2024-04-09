fn main() {
  new_inter();
  new_way_to_inter();
  let mut num = 6;
  loop {
    println!("{:?}", num);
    num = num - 1;
    if num == 0 {
      break;
    }
  }
  println!("done interating!")
}


fn new_inter(){
  for i in 1..=4 { // 1..=4 is a range inclusive operator that specifies that the range should include the end value(in this case 4). So 1..=4 means "start from 1 and go up to and including 4".
    println!("{:?}", i)
  }
  println!("yeah we done!")
}


fn new_way_to_inter() {
  let mut n = 1;
  loop {
    println!("{:?}", n);
    if n == 4 {
      break;
    }
    n = n + 1;
  }
  println!("we did it in a new way")
}