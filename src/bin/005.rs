use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [u8; k]
    };

    const MOD: usize = (1e9 + 7.0) as usize;
    let mut dp = vec![vec![b + 1; 10]; k + 1];
    for i in 0..k {
        for j in 0..b {
            
        }
    }
    let ans = 0;

    println!("{}", ans);
}
