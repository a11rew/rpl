fn main() {
  let str_ref = return_ref();

  change(str_ref)
}

fn return_ref() -> &String {
  let s = "string";
  &s
}

fn change(some_string: &String) {
  // some_string.push_str(", world");

  println!("Mutated {}", some_string)
}