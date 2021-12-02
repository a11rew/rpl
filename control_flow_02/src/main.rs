fn main() {
    labelled_loop();
    for_loop();
    range_loop()
}

fn range_loop() {
    // This syntax is pretty cool considering py has the range function and js doesn't even have it builtin
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }
}

fn labelled_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
