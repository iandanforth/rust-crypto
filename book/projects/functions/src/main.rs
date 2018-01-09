fn main() {

	let mut y = 6; // This is a STATEMENT, statements do not return values. However the '6' part is an expression as it evaluates/returns 6.
	y = five();
	y = plus_one(y);

	// let x = let y = 6 <-- This wouldn't work like in python because statements don't return values

	// Here we have an expression {} that creates a new scope, it returns a value
	let z = {
		let p = 7;
		2 + p // This is an expression (NOTE LACK OF ;) that returns a value and will be bound to z.
	};

    another_function(y, z);
}

// Nope this is illegal
// {
// 	3 + 5 // Raw expression in brackets ok?
// }

fn another_function(x: i32, y: i32) {
	println!("The value of x is {}", x);
	println!("The value of y is {}", y);
}

// Declare a function with a return value
fn five() -> i32 {
	5 // Again a raw expression without a semi colon returns the value
}

fn plus_one(x: i32) -> i32 {
	x + 1
}