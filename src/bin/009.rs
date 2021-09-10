use proconio::{fastout, input};
use std::f64::consts::PI;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n]
    };

    let mut ans = 0.0f64;
    let norm = |a: (f64, f64)| { (a.0.powi(2) + a.1.powi(2)).sqrt() };
    for i in 0..n {
        let (o1, o2) = xy[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            for k in 0..n {
                if i == k || j == k {
                    continue;
                }
                let (a1, a2) = xy[j];
                let (a1, a2) = (a1 - o1, a2 - o2);
                let (b1, b2) = xy[k];
                let (b1, b2) = (b1 - o1, b2 - o2);
                let cos_theta = (a1 * b1 + a2 * b2) / (norm((a1, a2)) * norm((b1, b2)));
                let theta = cos_theta.acos();
                let deg = theta * 180.0 / PI;
                // eprintln!("/P{}P{}P{} = {}[deg]", i, j, k, deg);
                ans = ans.max(deg);
            }
        }
    }

    println!("{}", ans);
}
