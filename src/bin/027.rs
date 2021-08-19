use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    };

    let mut set = HashSet::new();
    for (s, i) in s.into_iter().zip(1..) {
        if !set.contains(&s) {
            println!("{}", i);
        }
        set.insert(s);
    }
}
