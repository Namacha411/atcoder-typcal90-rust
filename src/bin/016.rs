use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize
    };

    const MAX: usize = 10000;
    let mut ans = MAX;
    for x in 0..MAX {
        for y in 0..(MAX - x) {
            if n < a*x + b*y { continue; } 
            if (n - a*x - b*y) % c != 0 { continue; }
            let z = (n - a*x - b*y) / c;
            ans = ans.min(x + y + z);
        }
    }

    println!("{}", ans);
}