use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n]
    };

    let cnt = a
        .into_iter()
        .zip(b)
        .map(|(a, b)| (a - b).abs())
        .sum::<isize>();
    let ans = if (cnt <= k) && ((k - cnt) % 2 == 0) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
