fn main() {
    // rust ownership
    // heap slow need allocation time, search etc...
    // stack fast, types are known, alloc doesnt need to search

    // value has owner
    // only one owner
    // no scope, no value -> memory safety!

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    // move example. s1 moves to s2
    // s1 pointer, len, capacity is moved to s2
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    // this example leaves  the original hello heap out of scope so it is dropped.
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    //example of clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Copy trait can only be applied to stack stuff
    // like int, bool, float, char, tuplles

    // here we see how ownership happens with functions
    let s = String::from("bubba"); // s comes into scope
    println!("before function uses s : {}", s);

    takes_ownership(s); // s's value moves into the function...
    // println!("after function uses s : {}", s); this will not compile. s is gone

                        // ... and so is no longer valid here

    let x = 109; // x comes into scope

    makes_copy(x); // x would move into the function,
    println!("after function uses x : {}", x);
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} //some integer goes out of scope
