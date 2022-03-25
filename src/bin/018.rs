use std::f64::consts::PI;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e: [f64; q]
    }

    for e in e {
        let z = l / 2.0 - l * (- 2.0 * PI * e / t).cos() / 2.0;
        let y = y - l * (- 2.0 * PI * e / t).sin() / 2.0;
        let norm = (x.powi(2) + y.powi(2)).sqrt();
        let theta = (z / norm).atan();
        let ans = theta * 180.0 / PI;

        println!("{}", ans);
    }
}