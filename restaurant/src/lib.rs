mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute Path
// use crate::front_of_house::hosting;

// Relative Path
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();

    let mut breakfast_meal = back_of_house::Breakfast::summer("Rye");

    breakfast_meal.toast = String::from("Bread");

    let _appetizer = back_of_house::Appetizer::Soup;

    // There is a better way to do this!
    // front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn _serve_order() {}

mod start_of_hosue {
    pub mod hosting {
        pub fn _add_to_waitlist() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // By default if you mark enums public, then it will make all the variants public
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

    fn cook_order() {}

    fn _fix_incorrect_order() {
        cook_order();
        // Use super to access the functions in the parent module
        super::_serve_order();
    }
}
