use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String
    };

    const MOD: u64 = (1e9 + 7.0) as u64;
    let atcoder = "atcoder".chars().collect::<Vec<_>>();
    let alen = atcoder.len();
    let s = s.chars().collect::<Vec<_>>();
    let slen = n;
    // let mut dp = vec![vec![0u64; alen + 1]; slen + 1];
    let mut dp = [[0; 8]; 100_000];
    dp[0][0] = 1;
    for i in 0..slen {
        for j in 0..=alen {
            dp[i + 1][j] += dp[i][j];
            if j < alen && s[i] == atcoder[j] {
                dp[i + 1][j + 1] += dp[i][j];
            }
        }
        for j in 0..=alen {
            dp[i + 1][j] %= MOD;
        }
    }
    let ans = dp[slen][alen];

    println!("{}", ans);
}
