// Nested paths
use rand::{Rng, CryptoRng};

// self is also short for the parent module itself
// Example: std::io::{self} = use std::io;
use std::io::{self, Write};

// * allows to bring all of the items from a module in scope.
use std::fmt::*;


// By default child modules are private, 
// However child modules can see the parent modules 
mod front_of_house;


// super allows us to go up one level in the module tree
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad
    }
}


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path start from the current module
    front_of_house::hosting::add_to_waitlist();
}

mod breakfast_boh {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
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

// enum variants inherit the privacy rules of an enum. 
// If the enum is public so are the variants, vice versa.
pub fn eat_at_breakfast_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
}


// We can use the "use" keyword to bring a path into scope.
// self references the current module
use self::front_of_house::hosting;

pub fn eat_at_restaurant_use() {
    hosting::add_to_waitlist();
}

// allows us to export a path from a module for external usage
pub use crate::front_of_house::hosting;