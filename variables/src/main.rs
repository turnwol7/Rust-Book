fn main() {
    // example showing shadowing and scope
    let x: i32 = 5;

    let x: i32 = x + 1;
    {
        let x: i32 = x * 2;
        //inner scop x = 12
        println!("the value of x inner scope is {x}");
        
    }
    // outer scope x = 6
    println!("the value of x is {x}");

}