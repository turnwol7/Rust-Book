fn main() {
let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!}

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

}
// example of the long way around. we look for the first space and return 5 for hello
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}