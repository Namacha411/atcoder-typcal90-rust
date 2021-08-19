use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize
    };

    let ans = if w == 1 || h == 1 {
        w * h
    } else {
        (h / 2 + h % 2) * (w / 2 + w % 2)
    };

    println!("{}", ans);
}
