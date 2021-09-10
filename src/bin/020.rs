use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        a: u128,
        b: u32,
        c: u128
    };

    let left = a;
    let right = c.pow(b);
    let ans = if left < right {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
