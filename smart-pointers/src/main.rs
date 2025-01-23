
//Smart pointers here are data structures with extra metadata and capabilities
//Box<T> allows to store data on heap rather than on stack
//Rc<T>
//Ref<T>
use std::ops::Deref;

// our custom pointer struct
struct MyBox<T>(T);

// implementation of the class that can dereference too~!
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}


// use our custom pointer to make a new string, then dereference it using the cusomt class that uses Deref to print the name rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}