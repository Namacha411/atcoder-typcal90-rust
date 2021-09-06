use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
        k: usize
    };

    let to_nine = |mut x: u128| {
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
    let to_ten = |mut x: u128| {
        let mut vec = vec![];
        while x != 0 {
            vec.push(x % 10);
            x /= 10;
        }
        let mut res = 0;
        for (i, x) in vec.into_iter().enumerate() {
            res += x * 8u128.pow(i as u32);
        }
        res
    };
    let mut n = n;
    for _ in 0..k {
        n = to_nine(to_ten(n))
            .into_iter()
            .map(|x| if x == '8' { '5' } else { x })
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
    }
    let ans = n;

    println!("{}", &ans);
}
