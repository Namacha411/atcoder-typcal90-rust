use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        q: usize,
        b: [isize; q],
    };

    let mut a = a;
    a.sort_unstable();
    for b in b.iter() {
        let index = upper_bound(&a, *b);
        let ans = if index == 0 {
            (a[index] as isize - *b as isize).abs()
        } else if index == n {
            (a[index - 1] as isize - *b as isize).abs()
        } else {
            (a[index] as isize - *b as isize)
                .abs()
                .min((a[index - 1] as isize - *b as isize).abs())
        };

        println!("{}", ans);
    }
}

pub fn upper_bound<T>(iterable: &[T], key: T) -> usize
where
    T: Ord,
{
    let mut first = 0;
    let mut length = Some(iterable.len());

    while let Some(len) = length {
        if len == 0 {
            break;
        }
        let half = len / 2;
        let middle = first + half;
        if key < iterable[middle] {
            length = Some(half);
        } else {
            first = middle;
            first += 1;
            length = len.checked_sub(half + 1);
        }
    }

    first
}
