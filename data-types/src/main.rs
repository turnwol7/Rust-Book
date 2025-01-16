use std::io;


fn main() {

    //scalar types = integer, floating point, numbers , booleans and characters

    // integer overflow
    // provides complement wrapping but not advised as value will be wrong

    // ⚡️ ✅ ☕️ 👍

    let emoji = '💰';
    println!("{emoji}");


    // can destructure to get the item out of the tuple.
    let tup  = ('✅', 5.5, 4);
    let (x, y, z) = tup;


    println!("{x}");

    // can also use decimal to get iteam
    let check = tup.0;
    println!("{}",check);


    // or arrays 
    let a: [u8; 4] = [1,2,3,4];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    //if you select item outside range you crash with runtime over

    // possible error input must be at run time. outlinging Rusts memory safety principles in action.
}
