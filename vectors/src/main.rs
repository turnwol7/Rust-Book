fn main() {

    //vectors. same type, contiguous, dynamic size
    let v: Vec<i32> = Vec::new();

    // i32 is default int type
    let mut v2 = vec![1,2,3];

    //push 
    v2.push(4);
    v2.push(5);
    v2.push(6);

    let third: &i32 = &v2[2];
    println!("print third element {}", third);

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!(" third element {}", third),
        None => println!("there is no third element"),
    }

    // get immutable references in loop
    let c = vec![1,2,3,4,5];
    for i in &c {
        println!("{i}");
    }

    let mut z = vec![100, 32, 57];
    for i in &mut z {
        *i += 50;
        println!("new {i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}