// Rust’s patterns are very useful in distinguishing between different kinds of data. When used in match expressions, Rust ensures your patterns cover every possible value, or your program won’t compile. Patterns in let statements and function parameters make those constructs more useful, enabling the destructuring of values into smaller parts at the same time as assigning to variables. We can create simple or complex patterns to suit our needs.

// https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html


enum Message {
    Hello { id: i32 },
}


fn main() {
let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {

        // matches in range
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {id_variable}"),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {id}"),
}
}
