extern crate csv;
extern crate nalgebra as na;

// Linear Regression Translation from matlab into rust
use std::fs::File;
use std::iter::Sum;
use na::{DMatrix, Vector2, U1, U2};

fn fit() {
    println!("Fitting data ...");
}

fn load(filename: &'static str) -> na::Matrix<f64, na::Dynamic, na::Dynamic, na::MatrixVec<f64, na::Dynamic, na::Dynamic>> {
    let file = File::open(filename).expect("Blur blur");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut all_vals = Vec::new();
    let mut col_count = 0;
    let mut row_count = 0;
    let mut temp_count = 0;

    for result in rdr.records() {
        let record = result.expect("Bad line");
        for field in record.iter() {
            let f = field.parse::<f64>().expect("fluber");
            all_vals.push(f);
            temp_count += 1;
        }
        if col_count == 0 {
            col_count = temp_count;
        }
        row_count += 1;
    }

    println!("{:?}", &row_count);
    let dmatrix = DMatrix::from_row_slice(
        row_count,
        col_count,
        &all_vals
    );

    return dmatrix;
}

fn sum(vec: na::Matrix<f64, na::Dynamic, na::U1, na::MatrixVec<f64, na::Dynamic, na::U1>>) -> f64 {
    let mut total = 0.0;
    for val in vec.iter() {
        total += val;
    }

    return total;
}

fn compute_cost(
        X: na::Matrix<f64, na::Dynamic, na::U2, na::MatrixVec<f64, na::Dynamic, na::U2>>,
        y: na::Matrix<f64, na::Dynamic, na::U1, na::MatrixVec<f64, na::Dynamic, na::U1>>
    ) -> f64 {

    let m = y.len();
    let scalar = (1.0 / (2.0 * m as f64));
    println!("{:?}", &scalar);
    let theta = Vector2::new(0.0, 0.0);
    let prod = X * theta;
    let err = prod - y;
    println!("{:?}", &err);
    let sqr = power(err, 2);
    let total = sum(sqr);
    println!("{:?}", &total);
    let J = scalar * total;
    return J;
}

fn power(
    mut vec: na::Matrix<f64, na::Dynamic, na::U1, na::MatrixVec<f64, na::Dynamic, na::U1>>,
    exp: i32) -> na::Matrix<f64, na::Dynamic, na::U1, na::MatrixVec<f64, na::Dynamic, na::U1>> {

    for ind in 0..vec.len() {
        vec[ind] = vec[ind].powi(exp);
    }

    return vec;
}

fn main() {
    // Set up data
    let data = load("ex1data1.txt");
    // Pull out the first two columns then add a 1's column
    let X = data.column(0).insert_column(0, 1.0);
    // Pull out the final column as the labels
    let y = data.column(1).clone_owned();
    println!("{:?}", &y);

    let theta = Vector2::new(0.0, 0.0);

    let iterations = 1500;
    let alpha = 0.01;

    let cost = compute_cost(X, y);
    println!("{:?}", cost);

    fit();
}
