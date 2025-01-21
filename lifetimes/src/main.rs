use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    
}

// parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause. This extra parameter will be printed using {}, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// generic lifetime type, and T because you need them for Display Trait 
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    // ann of type T
    ann: T,
) -> &'a str
// returns lifetime str where it needs Display Trait
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}