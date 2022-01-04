mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    //'use' for path
    add_to_waitlist();
    add_to_waitlist();
}

mod back_of_house {

    pub enum Appetizer {
        Soup, 
        Salad,
    }
    // public enum implies everything inside is public

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from('peaches'),
            }
        }
    }
}


pub fn eat_at_restaurant_back() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;   
}