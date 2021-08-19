use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: String
    };

    let ss = &s.chars().sorted().collect::<String>()[0..k];
    let ans = &s.chars().filter(|c| ss.contains(*c)).collect::<String>()[0..k];

    print!("{}", ans);
}
