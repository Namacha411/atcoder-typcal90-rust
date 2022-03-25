use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let angle = |point: (f64, f64)| -> f64 {
        let (x, y) = point;
        let i = x / (x.powi(2) + y.powi(2)).sqrt();
        let angle = i.acos() * 180.0 / PI;
        if y >= 0.0 { angle } else { 360.0 - angle }
    };
    let mut ans = 0.0f64;
    for i in 1..=n {
        let res = 0.0;
        let mut v = vec![0;0];
        for j in 1..=n {
            if i == j {
                continue;
            }
        }
        ans = ans.max(res);
    }

    println!("{}", ans);
}
