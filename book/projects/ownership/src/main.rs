fn foobar() {
	println!("This says bar");
}


fn main() {

	foobar(); // Prints 'this says foo'

	// Apparently functions get hoisted?
	fn foobar() {
		println!("This says foo");
	}

	foobar();

	let mut s = String::from("hello");

	s.push_str(", world.");

	println!("{:?}", s);

	let s1 = String::from("foo");
	let s2 = s1.clone(); // Clone is required if 'moving' data to preserve 1 owner policy

	println!("{:?}", s1);

	let x = 5;
	let y = x; // This is a special case where a move doesn't happen.

	println!("x = {}, y = {}", x, y);

	/////

	let t = String::from("Bianca is cute!");

	// I kinda like how values only exist in one scope at a time
	// What does this mean for closures?
	take_ownership(t);

	//println!("{:?}", t); // This would throw an error because t has moved


}

fn take_ownership(some_string: String) {
	println!("{:?}", some_string);
}
