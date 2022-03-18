use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [isize; n],
        b: [isize; n]
    };

    let mut a = a;
    a.sort_unstable();
    let mut b = b;
    b.sort_unstable();
    let ans = a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).sum::<isize>();

    println!("{}", ans);
}
