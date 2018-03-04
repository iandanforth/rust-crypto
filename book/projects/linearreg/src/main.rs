extern crate csv;

// Linear Regression Translation from matlab into rust
use std::fs::File;

fn fit() {
    println!("Fitting data ...");
}

fn load(filename: &'static str) {
    let file = File::open(filename).expect("Blur blur");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.expect("Bad line");
        println!("{:?}", record);
    }

}

fn main() {
    // Set up data
    load("ex1data2.txt");
    fit();
}
