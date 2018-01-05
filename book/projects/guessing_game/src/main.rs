extern crate rand; // Equivalent to import in python

// Not much is brought in during the prelude, include standard library input/output here
use std::io;
use rand::Rng; // this is a 'trait' which must be in scope for random generating method to work

fn main() {
	// Remember the ! means this is a macro
    println!("Guess the number!");

    let mut rand_generator = rand::thread_rng(); // If this isn't mutable it throws an error. No idea why.
    let secret_number = rand_generator.gen_range(1, 101); // Half open crap
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");
    // Create a mutable variable named guess and assign it an instance of a new String object
    let mut guess = String::new();
    // let mut guess = ""; This doesn't work ... no idea why
    io::stdin().read_line(& mut guess)
    	.expect("Failed to read line.");

    println!("You guessed {}", guess);
}
