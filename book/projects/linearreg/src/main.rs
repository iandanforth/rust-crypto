extern crate csv;
extern crate nalgebra as na;


// Linear Regression Translation from matlab into rust
use std::fs::File;
use na::{Matrix, Dynamic, MatrixArray, MatrixVec, RowDVector};

fn fit() {
    println!("Fitting data ...");
}

fn load(filename: &'static str) {
    type DMatrixi32 = Matrix<i32, Dynamic, Dynamic, MatrixVec<i32, Dynamic, Dynamic>>;
    let file = File::open(filename).expect("Blur blur");
    let mut rdr = csv::Reader::from_reader(file);
    let mut rows = Vec::new();
    let mut colCount = 0;
    for result in rdr.records() {
        let record = result.expect("Bad line");
        let mut temp = Vec::new();
        for field in record.iter() {
            let f = field.parse::<i32>().expect("fluber");
            temp.push(f);
        }
        if colCount == 0 {
            colCount = temp.len();
        }
        let rdv = RowDVector::from_iterator(3, temp.iter().cloned());
        rows.push(rdv);
    }
    let rowCount = rows.len();
    if 0.0 == 0 {
        println!("I am a sane programming language");
    }
    // let m = DMatrixi32::from_iterator(rowCount, colCount, rows.iter().cloned());


}

fn main() {
    // Set up data
    load("ex1data2.txt");
    fit();
}
