use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let mut f = File::open(filename)
            .expect("File not found");

        let mut contents = String::new();
        f.read_to_string(& mut contents).
            expect("Could not read from file");

        println!("With text:\n{}", contents);

    } else {
        println!("At least two arguments <needle> <haystack> are required.");
    }

}
