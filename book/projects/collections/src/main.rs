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


    // Some string shit
    // A String is a wrapper over a Vec<u8>
    // Rust encodes strings incorrectly and can't handle basic indexing
    // This is because rust is concerned more with how things are stored in memory than making the language useable 
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Hash maps

    // The fact that this is not included in the prelude is a crime
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // Interestingly this won't compile until you insert something so it can infer types
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Red"), String::from("A lot!")); // This is a compile error

    // You then use .get() to retrieve values, the return is an Option<&V> which means it can be None
    let blue_score = scores.get(&String::from("Blue")); // This has to be a reference, but it's cool if the reference is anonymous

    // There is also a 'collect' method which will turn tuples into a hashmap
    let teams  = vec![String::from("Blue"), String::from("Yellow")]; // Make a vector of strings
    let initial_scores = vec![10, 50]; // Make a vector of ints
    // Call zip() on the iterable version of teams, passing in the iterable version of scores and then call collect on the resulting set of tuples
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); 

    let mut more_scores = HashMap::new();

    // Modifying existing values
    more_scores.insert(String::from("Blue"), 20);
    more_scores.insert(String::from("Blue"), 10); // <- This value wins (as expected)
    println!("{:?}", more_scores);

    // If you want to conditionally insert (only if a value doesn't exist)
    more_scores.entry(String::from("Yellow")).or_insert(30); // Key doesn't exist so the key/value pair is created
    more_scores.entry(String::from("Yellow")).or_insert(10); // Key DOES exist so the value isn't modified.
    println!("{:?}", more_scores );

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

