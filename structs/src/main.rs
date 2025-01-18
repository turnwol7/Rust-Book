fn main() {
    let user1 = User {
        active: true,
        username: String::from("someone1@email.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheruser@email.com");

    // NO struct update syntax 
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    // WITH struct update syntax
    let user3 = User {
        email: String::from("anotherone@email.com"),
        ..user1
    };

    // instacnes of stcut tubples 
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //unit like structs
    let subject = AlwaysEqual;

}

// make user using the user struct (field init shorthand)
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// struct update syntax



struct User {
    active: boolean,
    username: String,
    email: String,
    sign_in_count: u64,
}

// struct tuples

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// unit like structs
struct AlwaysEqual;