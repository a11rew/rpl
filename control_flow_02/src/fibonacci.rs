// Generates nth fibonacci number
use std::io;

fn main() {
  let mut input = String::new();
  let mut count = 0;

  println!("Please enter number of iterations");
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  let max: i32 = match input.trim().parse() {
    Ok(val) => val,
    Err(_) => panic!("enter a nuber"),
  };

  // seed values
  let mut previous_0 = 0;
  let mut previous_1 = 1;

  while count < max {
    if count == 0 {
      println!("Fibonacci sequence index {} is {}", count, previous_0);
      count = count + 1;
      continue;
    }

    if count == 1 {
      println!("Fibonacci sequence index {} is {}", count, previous_1);
      count = count + 1;
      continue;
    }

    let fib: i64 = previous_0 + previous_1;
    println!("Fibonacci sequence index {} is {}", count, fib);

    count = count + 1;
    previous_0 = previous_1;
    previous_1 = fib;
  }
}
