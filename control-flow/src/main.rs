fn main() {
    fizzy(100);
}

fn fizzy(input: u32) {
    let mut number = input;

    while number > 0 {

        if (number % 5 == 0) && (number % 3 == 0) {
            println!("fizzbuzz");
        } else if number % 5 == 0 {
            println!("fizz");
        } else if number % 3 == 0 {
            println!("buzz");
        } else {
            println!("{}", number);
        }

        number -= 1;

    }
}