use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String
    };

    let mut memo = vec![vec![0; s.len() + 1]; 26];
    let s = s.chars().map(|c| c as u8 - b'a').collect::<Vec<_>>();
    for memo in memo.iter_mut() {
        memo[s.len()] = s.len();
    }
    for i in (0..s.len()).rev() {
        for j in 0..26 {
            memo[j][i] = if s[i] as usize == j {
                i
            } else {
                memo[j][i + 1]
            };
        }
    }
    let mut ans = "".to_string();
    let mut now = 0;
    for i in 0..k {
        for j in 0..26 {
            let next = memo[j][now];
            let maxlen = i + s.len() - next;
            if maxlen >= k {
                ans += &((j as u8+ b'a') as char).to_string();
                now = next + 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
