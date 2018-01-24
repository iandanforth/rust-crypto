// The name lib.rs is not special. You can also put modules in your main.rs if you like



mod network {
    fn connect() {

    }
}

// Not sure why this is allowed or encouraged but ok
mod client {
    fn connect() {

    }
}

// This won't compile, module names must be unique in a given namespace.
// mod client {

// }

// This is dumb but compiles just fine and dandy
mod outside {

    fn foo() {
    }

    mod inside {
        fn foo() {

        }
    }
}

mod client2; // If clinet2 doesn't exist as client2.rs or mod/client2.rs it won't compile


#[cfg(test)]
mod tests { // Mod keyword defines this as a module
    #[test]
    fn it_works() { // Functions are by default private
        assert_eq!(2 + 2, 4);
    }
}
