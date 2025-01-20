// you can make variants use other enums
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


// you can put match statements in functions
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        },
    }
}

// you can match Some with actions
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5); // option of some 5
    let six = plus_one(five); // takes option, matches option 
    let none = plus_one(None); // mataches empty value

    //can match funcitons
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll(),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn re_roll() {}