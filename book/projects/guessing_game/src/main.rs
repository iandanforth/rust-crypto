// Not much is brought in during the prelude, include standard library input/output here
use std::io;

fn main() {
	// Remember the ! means this is a macro
    println!("Guess the number!");
    println!("Please input your guess.");
    // Create a mutable variable named guess and assign it an instance of a new String object
    let mut guess = String::new();
    // let mut guess = ""; This doesn't work ... no idea why
    io::stdin().read_line(& mut guess)
    	.expect("Failed to read line.");

    println!("You guessed {}", guess);
}
