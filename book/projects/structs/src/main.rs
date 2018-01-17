fn main() {
    // Structs are like objects
    // They are also like rust tuples in that each value can be a different type
    // The key:type pairs that make up a struct are called 'fields'

    // I wonder if these can have default values as well
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // To use a struct we need to create an instance
    let user = User {
        username: String::from("Tim Johnson"),
        email: String::from("tjthemax@gmail.com"),
        sign_in_count: 33,
        active: false,
    };

    // Yay dot notation on structs works!
    println!("{}", user.username);

    let mut user2 = User {
        username: String::from("Jane Blue"),
        email: String::from("shethebest@gmail.com"),
        sign_in_count: 99,
        active: true
    };

    user2.username = String::from("Jane 'The Bomb' Blue");

    println!("{:?}", user2.username);

    fn new_user(username: String, email: String) -> User {
        let nu = User {
            username: username, // Ugh this also supports javascript like "shorthand". No fucking way.
            email: email,
            sign_in_count: 0,
            active: true
        };
        return nu;
    }

    let bob = new_user(String::from("Bob Vila"), String::from("bob@thisoldhouse.com"));

    // You can also do partial struct expansion with .. syntax
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };

    // Named tuples are called 'tuple structs' and look like this
    struct Boople(String, i32, u64);
    // They have no field names, just types
    // You can access the values still using dot notation and indices
    let b = Boople(String::from("Boop!"), 5, 20);
    println!("{:?}", b.0);


}
