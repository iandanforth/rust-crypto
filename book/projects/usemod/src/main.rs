pub mod a {
    pub mod series {
        pub mod of{
            pub fn nested_modules() {}
        }
    }
}

use a::series::of; // Bring the 'of' mod scope into scope of this script

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}

// from foo import (baz, blor. norp)
// * is also allowed - boo
use TrafficLight::{Red, Yellow}; // You can include multiple names from a namespace/enum with this syntax

mod nested {
    fn connect() {}
    mod craziness {
        use super::connect; // A module does not immediately have access to its parents scope, this brings it in
        use super::*; // You can also bring things in multiple times without issue
        fn thing() {
            connect();
        }
    }
}

fn main() {
    a::series::of::nested_modules(); // Call using full nested path
    of::nested_modules(); // Call from use context

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
