use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    };

    let to_nine = |mut x: usize| {
        let mut vec = vec![];
        while x != 0 {
            vec.push(x % 9);
            x /= 9;
        }
        vec.iter()
            .map(|x| (*x as u8 + '0' as u8) as char)
            .rev()
            .collect_vec()
    };
    let to_ten = |mut x: usize| {
        let mut vec = vec![];
        while x != 0 {
            vec.push(x % 10);
            x /= 10;
        }
        let mut res = 0;
        for (i, x) in vec.into_iter().enumerate() {
            res += x * 8usize.pow(i as u32);
        }
        res
    };
    let mut n = n;
    for _ in 0..k {
        n = to_nine(to_ten(n))
            .into_iter()
            .map(|x| if x == '8' { '5' } else { x })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }
    let ans = n;

    println!("{}", &ans);
}
