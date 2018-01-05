extern crate rand; // Equivalent to import in python

// Not much is brought in during the prelude, include standard library input/output here
use std::io;
use std::cmp::Ordering;
use rand::Rng; // this is a 'trait' which must be in scope for random generating method to work

fn main() {
	// Remember the ! means this is a macro
    println!("Guess the number!");

    let mut rand_generator = rand::thread_rng(); // If this isn't mutable it throws an error. No idea why.
    let secret_number = rand_generator.gen_range(1, 101); // Half open crap
    // println!("The secret number is {}", secret_number);
 
    // let mut guess = String::new(); Having this outside the loop doesn't work for some reason
    // crashes with 'ParseIntError { kind: InvalidDigit }'

    loop {
	    println!("Please input your guess.");
	    // let mut guess = ""; This doesn't work ... no idea why

	    // Create a mutable variable named guess and assign it an instance of a new String object
	    let mut guess = String::new();

	    io::stdin().read_line(& mut guess)
	    	.expect("Failed to read line.");

	    let guess_num: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
	    	Err(_) => , // What is _ and how do I print out the error text?
	    };
	    	// .expect("Please enter a number."); // This doesn't seem to create a clean error and crash like in python there is other distracting messaging
	    	// Also you'll need to know how to either handle or prevent inputs which overflow this type

	    println!("You guessed {}", guess_num);

	    match guess_num.cmp(&secret_number) {
	    	Ordering::Less => println!("Too small!"),
	    	Ordering::Greater => println!("Too big!"),
	    	Ordering::Equal => {
	    		println!("That's it! Nice work.");
	    		break;
	    	}
	    }
    }

}
