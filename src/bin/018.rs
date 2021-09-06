use std::f64::consts::PI;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        _y: f64,
        q: usize,
        e: [f64; q]
    }

    for e in e {
        let x = l * (2.0 * PI * e / t).cos();
        let y = l * (2.0 * PI * e / t).sin();
        eprintln!("(x, y) = ({}, {})", x, y);
    }
}