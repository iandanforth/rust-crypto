extern crate csv;
extern crate nalgebra as na;

// Linear Regression Translation from matlab into rust
use std::fs::File;
use std::iter::Sum;
use na::{DMatrix, Vector2};

fn fit() {
    println!("Fitting data ...");
}

fn load(filename: &'static str) -> na::Matrix<f64, na::Dynamic, na::Dynamic, na::MatrixVec<f64, na::Dynamic, na::Dynamic>> {
    let file = File::open(filename).expect("Blur blur");
    let mut rdr = csv::Reader::from_reader(file);
    let mut all_vals = Vec::new();
    let mut col_count = 0;
    let mut row_count = 0;
    let mut temp_count = 0;
    for result in rdr.records() {
        let record = result.expect("Bad line");
        for field in record.iter() {
            println!("{:?}", field);
            let f = field.parse::<f64>().expect("fluber");
            all_vals.push(f);
            temp_count += 1;
        }
        if col_count == 0 {
            col_count = temp_count;
        }
        row_count += 1;
    }

    let dmatrix = DMatrix::from_iterator(
        row_count,
        col_count,
        all_vals.iter().cloned()
    );

    return dmatrix;
}

fn compute_cost(
        X: na::Matrix<f64, na::Dynamic, na::Dynamic, na::MatrixVec<f64, na::Dynamic, na::Dynamic>>,
        y: na::Matrix<f64, na::Dynamic, na::Dynamic, na::MatrixVec<f64, na::Dynamic, na::Dynamic>>
    ) -> f64 {

    let m = y.len();
    let scalar = 1 / (2 * m);
    let theta = Vector2::new(0.0, 0.0);
    let prod = X * theta;
    println!("{:?}", prod); 
    // J = (1/(2*m)) * sum(((X * theta) - y).^2);
    return 0.0;
}

fn main() {
    // Set up data
    let data = load("ex1data1.txt");
    // Pull out the first two columns then add a 1's column
    let X = data.columns(0, 1).insert_column(0, 1.0);
    // Pull out the final column as the labels
    let y = data.columns(1, 1).clone_owned();

    let theta = Vector2::new(0.0, 0.0);

    let iterations = 1500;
    let alpha = 0.01;

    compute_cost(X, y);

    fit();
}
