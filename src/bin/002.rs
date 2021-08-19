use std::collections::BTreeSet;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let ok = |x: usize| -> bool {
        let mut que = 0isize;
        for i in 0..n {
            if que < 0 {
                return false;
            }
            if (x >> i) & 1 == 0 {
                que += 1;
            } else {
                que -= 1;
            }
        }
        que == 0
    };
    let out = |x: usize| -> String {
        let mut ans = String::new();
        for i in 0..n {
            if (x >> i) & 1 == 0 {
                ans += "(";
            } else {
                ans += ")";
            }
        }
        ans
    };
    let mut set = BTreeSet::new();
    for i in 0..(1 << n) {
        if ok(i) {
            set.insert(out(i));
        }
    }

    for ans in set {
        println!("{}", ans);
    }
}
