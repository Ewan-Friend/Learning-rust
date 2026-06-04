mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn cook_order() {}

    /*
     * ----- Using Super -----
     */

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    
    /*
     * ----- Make Structs and Enums Public -----
     */

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Constructor is needed as "seasonal_fruit" could not be written to elsewise
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn deliver_order() {}

/*
 * ----- Module Paths  -----
 */


pub fn eat_at_restaurant() {

    // Absolute path 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    
    meal.toast = String:: from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

/*
 * ----- Bringing Paths To Scope -----
 */

pub use crate::front_of_house::hosting; // External code can use
                                        // restaurant::hosting::add_to_waitlist()

pub fn eat_at_restaurant_() {
    hosting::add_to_waitlist();
}

/*
 * ----- Providing New Names ----- 
 */

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result { ... }
// fn function2() -> IoResult<()> { ... }

/*
 * Nested Paths
 */

use std::{cmp::Ordering, io};

/*
 * Import Using Glob
 */

use std::collections::*;
