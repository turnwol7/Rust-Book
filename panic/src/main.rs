use std::net::IpAddr;

fn main() {
    // when to call panic??
    // can call panic wether recover or not
    // but return Result you give code Options what to do

    // can use unwrap if you never expect an err
    // unwrap() will panic on None
    // unwrap expects Some. this compiles
    let mut diggy = Some(String::from("boogy"));
    diggy.unwrap();

    // when hard coded? we can expect
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // good to return result than panic
    // need decisions.

}



// struct
pub struct Guess {
    value: i32,
}

impl Guess {
    // new returns a Guess
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // getter for field data
    pub fn value(&self) -> i32 {
        self.value
    }
}

// tldr use Result and Panic in various situations to make code reliable
