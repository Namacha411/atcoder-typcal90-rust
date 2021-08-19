use std::vec;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1]
    };

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let p = dfs(&g, 0, -1);
    let q = dfs(&g, p.1, -1);
    let ans = q.0 + 1;

    println!("{}", ans);
}

fn dfs(g: &Vec<Vec<usize>>, now: usize, from: isize) -> (usize, usize) {
    let mut res = (0, now);
    for to in &g[now] {
        if *to as isize == from {
            continue;
        }
        let (dist, vertex) = dfs(&g, *to, now as isize);
        res = res.max((dist + 1, vertex));
    }
    res
}