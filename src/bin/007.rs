use itertools::Itertools;
use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
        q: usize,
        b: [usize; q]
    };

    let a = a.into_iter().sorted().collect::<Vec<usize>>();
    let ok = |mid: usize| -> bool {
        true
    };
    for b in b {
        let (mut l, mut r) = (0, n);
        while r - l > 1 {
            let mid = (r + l) / 2;
            if ok(mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        let ans = l;

        println!("{}", ans);
    }
}
