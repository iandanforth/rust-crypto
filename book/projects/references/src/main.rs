fn main() {
    let s1 = String::from("Hello!");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    // change(&s1); This doesn't work because s1 was not orignally mutable

    let mut s2 = String::from("Foo ");
    change(& mut s2); // Here the reference must be specified as mutable AND the original variable must be mutable

    // Can we then change again?
    change(& mut s2);

    println!("{:?}", s2 );

    // Rust prevents more than one mutable reference from happening in the same scope
    // This prevents data races
    let r1 = & mut s2;
    // let r2 = & mut s2; NOT OK

    // Also not OK because r1 is still in scope.
    // {
    //     let r2 = & mut s2;
    // }

    let mut s3 = String::from("Bar");

    {
        let r3 = & mut s3; // r3 goes out of scope allowing for r4
    }

    let r4 = & mut s3;

    // Preventing dangling references
    // let ref_to_nothing = dangle();

    let normal_return = no_dangle();

    // Rules of references 

    // At any given time you can have either (but not both)
    // - one mutable reference
    // - any number of immutable references

    // References must always be valid

}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

// This doesn't work because you are trying to mutate a reference which is immutable by default
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: & mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("Dangle!");

//     return &s; // This doesn't work because s will immediately go out of scope and the reference will be invalid
// }

fn no_dangle() -> String {
    let s = String::from("No dangle");

    return s;
}
