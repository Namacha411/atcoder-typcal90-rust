use std::collections::VecDeque;

use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        q: usize,
        tx: [(usize, usize); q]
    };

    let mut deque = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => { deque.push_front(x) },
            2 => { deque.push_back(x) },
            3 => { println!("{}", deque[x-1]); },
            _ => { unreachable!() }
        }
    }
}
