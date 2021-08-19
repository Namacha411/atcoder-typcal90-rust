use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n]
    };

    let ok = |mid| -> bool {
        let mut prev = 0;
        let mut cut = 0;
        for a in a.iter() {
            if *a - prev >= mid && l - *a >= mid {
                prev = *a;
                cut += 1;
            }
        }
        cut >= k
    };
    let mut left = 0;
    let mut right = l;
    while right - left > 1 {
        let mid = (left + right) / 2;
        if !ok(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    let ans = left;

    println!("{}", ans);
}
