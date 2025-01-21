fn main() {
    
    let mut list = vec![34,78,34,34,78];

    list.sort();

    // find media of list vectors
    // if even or odd find middle
    let median = if list.len() % 2 == 0 {
        //even
        let mid1 = list[list.len() / 2 - 1];
        let mid2 = list[list.len() / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        //odd
        list[list.len() / 2] as f64
    };
    println!("{median}");

    
}
