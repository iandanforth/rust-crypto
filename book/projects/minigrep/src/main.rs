use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let mut f = File::open(filename)
        .expect("File not found");

    let mut contents = String::new();
    f.read_to_string(& mut contents).
        expect("Could not read from file");

    println!("With text:\n{}", contents);
}


// The rest of the exercise gives bad advice and should be ignored.
fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    return (query, filename);
}