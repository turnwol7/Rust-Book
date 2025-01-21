use std::error::Error;
use std::fs::File;

// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html


// massive chapter, follow link to re read bro


// Box can return any error 
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}