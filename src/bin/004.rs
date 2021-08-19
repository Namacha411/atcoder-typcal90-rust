use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    };

    let aw = a
        .iter()
        .map(|t| t.iter().sum::<usize>())
        .collect::<Vec<_>>();
    let ah = (0..w)
        .map(|i| (0..h).map(|j| a[j][i]).sum::<usize>())
        .collect::<Vec<_>>();
    let ans = (0..h)
        .map(|i| (0..w).map(|j| ah[j] + aw[i] - a[i][j]).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    for i in ans {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}
