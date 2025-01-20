fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // add bonk to adjusted string
    let s3 = "bonk";
    let s4 = s1 + s3;
    println!("{s4}");

    let s5 = "jungle";
    let s6 = s4 + s5;
    println!("s6 is {s6}");


    let hello = "Здравствуйте";

    let s = &hello[0..10];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    // unicode may be made of more than one byte
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // rust requires more though on strings compared to other languages
}
