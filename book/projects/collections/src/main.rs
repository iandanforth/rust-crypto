fn main() {

    let v: Vec<i32> = Vec::new(); // This returns a Vec<T> generic type and the type hint lets the compiler know what it will hold
    let v_with_macro = vec![1, 2, 3]; // use the vec! macro, which strangely has square brackets
    let another = vec!["hi there", "foobar", "morpmorp"];

    // Here's a random hack for finding out what the compiler inferred
    // another.what(); Call any unimplemented function and the compiler will spit out an error with the type
    // v_with_macro.what();

    // error[E0599]: no method named `what` found for type `std::vec::Vec<&str>` in the current scope
    //  --> main.rs:6:13
    //   |
    // 6 |     another.what();
    //   |             ^^^^

    // error[E0599]: no method named `what` found for type `std::vec::Vec<{integer}>` in the current scope
    //  --> main.rs:7:18
    //   |
    // 7 |     v_with_macro.what();
    //   |                  ^^^^

    let mut mutable_v = Vec::new(); // This is ok without the type hint because we push to it later?

    mutable_v.push(1);
    mutable_v.push(2);

    let mut v2 = vec![1, 2, 3, 4, 5];

    let with_index = v2[2];
    let with_get = v2.get(2);

    // let out_of_range = v2[200]; // Causes a panic
    let maybe_none = v2.get(200);

    println!("{:?}", maybe_none);

    // v2.push(6); This is an error because Rust isn't smart enough to update references for you if you change the size of an array


    // Iterating over vectors

    let new_v = vec![100, 32, 57];
    for i in &new_v {
        print!("{:?}\n", i);
    }

    println!("Now with mutable shit ....");
    let mut mut_new_v = vec![100, 32, 57];

    // The example encourages an anti-pattern which is mutating an object while iterating over it. NO BAD DON'T DO THAT.
    
    let mut holder: Vec<i32> = vec![];
    for mut i in mut_new_v {
        i += 50;
        holder.push(i); // Push into a new vector rather than mutating existing one.
        println!("{:?}", i);
    }
    // How to store multiple types in a vector using an enum
    // Create a little wrapper that goes through all the types
    enum w {
        i(i32),
        f(f64),
        s(String)
    }

    // Create a vector that is expected to all be of the type 'w' which we just created
    let mut holds_many: Vec<w> = vec![];

    holds_many.push(w::i(32));
    holds_many.push(w::s(String::from("Hi there!")));
    holds_many.push(w::f(22.22));


    println!("Hello, world!");
}
