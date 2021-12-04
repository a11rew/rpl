// Convert celcius to fahrenheit
use std::io;

fn main() {
  loop {
    println!("Enter the Celcius temp you want to convert to Fahrenheit ");
    let mut input = String::new();

    // Collect input in terminal
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
    let input: f32 = match input.trim().parse() {
      Ok(val) => val,
      Err(_) => {
        println!("enter a number");
        continue;
      }
    };

    fn convert_to_fahrenheit(value: f32) -> f32 {
      (value * 1.8) + 32 as f32
    }

    let fahrenheit_value = convert_to_fahrenheit(input);
    println!(
      "{} celcius converts to {} fahrenheit",
      input, fahrenheit_value
    );
    break;
  }
}
