fn main() {
    let s1 = String::from("hello");

    // takes reference from s1
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // make mutable
    let mut s = String::from("hellos");

    // change to accept mutable reference
    change(&mut s);

    println!("{s}");
    
        /////

    let mut t = String::from("balls");

    {
        let r1 = &mut t;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut t;

    // cant access r1, but yes to r2
    println!("{r2}");

    let mut z = String::from("hello");

    let r4 = &z; // no problem
    let r5 = &z; // no problem
    let r6 = &z; // no problem

    // cant have a immutable reference on a value and then next have a mutable reference of same one. first doesnt want things to change.

    println!("{}, {}, and {}", r4, r5, r6);

}

// takes in a reference string param
fn calculate_length(s: &String) -> usize {
    s.len()
}
// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is NOT dropped.

// chang to accept mutable reference
  fn change(some_string: &mut String) {
    some_string.push_str(", worldz");
}