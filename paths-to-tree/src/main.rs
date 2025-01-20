mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheate");
    println!("id like {} toast please", meal.toast);

    // we can access enums cause they are public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


    // this line fails because we try to accreess private meal fruit
    // meal.seasonal_fruit = String::from("blueberries");

}

fn main() {}