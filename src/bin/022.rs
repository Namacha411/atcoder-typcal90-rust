use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3]
    };

    let gcd = a.iter().fold(a[0], |acc, x| acc.gcd(x));
    let ans = a.iter().map(|x| (x / gcd) - 1).sum::<usize>();

    println!("{}", ans);
}
