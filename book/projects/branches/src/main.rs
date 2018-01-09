fn main() {
	let number = 6;

	if number % 4 == 0 {
		println!("number divisible by 4");
	} else if number % 3 == 0 {
		println!("number divisible by 3");
	} else if number % 2 == 0 {
		println!("number divisible by 2");
	} else {
		println!("number is not divisible by 4, 3 or 2");
	}

	let condition = true;

	let new_number = if condition { 5 } else { 6 }; // This seems valid, but probably not the best ternary

	// let broken = if condition { 5 } else { "foo "}; This doesn't compile. All branches need to be of the same type.
}
