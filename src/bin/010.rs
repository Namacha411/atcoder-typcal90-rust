use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q]
    };

    let lr = lr.into_iter().map(|(l, r)| (l - 1, r)).collect_vec();
    let mut csum1 = vec![0; n + 1];
    let mut csum2 = vec![0; n + 1];
    for (i, (c, p)) in (0..n).zip(cp) {
        if c == 1 {
            csum1[i + 1] = csum1[i] + p;
            csum2[i + 1] = csum2[i];
        }
        if c == 2 {
            csum2[i + 1] = csum2[i] + p;
            csum1[i + 1] = csum1[i];
        }
    }

    for (l, r) in lr {
        println!("{} {}", csum1[r] - csum1[l], csum2[r] - csum2[l]);
    }
}
