use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut vec = vec![0; n + 1];
    for (a, b) in ab {
        if a > b {
            vec[a] += 1;
        }
        if a < b {
            vec[b] += 1;
        }
    }
    let ans = vec.into_iter().filter(|x| *x == 1).count();

    println!("{}", ans);
}
