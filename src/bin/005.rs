use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        b: usize,
        k: usize,
        c: [u8; k]
    };

    const MOD: usize = (1e9 + 7.0) as usize;
    let ans = 0;
    let _problem1 = || {
        let mut dp = vec![vec![0; b + 1]; k + 1];
        for i in 0..k {
            for j in 0..k {
                for _k in 1..=k {
                    let nex = (j * 10 + c[i] as usize) % b;
                    dp[i + 1][nex] += dp[i][j];
                    dp[i + 1][nex] %= MOD;
                }
            }
        }
    };

    println!("{}", ans);
}
