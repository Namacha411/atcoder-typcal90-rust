use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    };

    let mut field = vec![vec![Color::White; w]; h];
    let mut uf = UnionFind::new(h * w);
    for _ in 0..q {
        input! { x: u8 };
        match x {
            1 => {
                input! {
                    r: usize,
                    c: usize,
                };
                let (y, x) = (r - 1, c - 1);
                field[y][x] = Color::Red;
                let dx = vec![-1, 0, 1, 0];
                let dy = vec![0, 1, 0, -1];
                let center = y * w + x;
                for (dx, dy) in dx.iter().zip(dy) {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0 || nx >= w as i32 {
                        continue;
                    }
                    if ny < 0 || ny >= h as i32 {
                        continue;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);
                    if field[ny][nx] == Color::White {
                        continue;
                    }
                    let t = ny * w + nx;
                    uf.merge(center, t);
                }
            }
            2 => {
                input! {
                    ra: usize,
                    ca: usize,
                    rb: usize,
                    cb: usize,
                };
                let (ya, xa) = (ra - 1, ca - 1);
                let (yb, xb) = (rb - 1, cb - 1);
                if field[ya][xa] == Color::White || field[yb][xb] == Color::White {
                    println!("No");
                    continue;
                }
                let pa = ya * w + xa;
                let pb = yb * w + xb;
                let ans = if uf.same(pa, pb) { "Yes" } else { "No" };
                println!("{}", ans);
            }
            _ => unreachable!(),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Red,
}

pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.parent(a);
        let mut y = self.parent(b);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.parent(a) == self.parent(b)
    }

    pub fn parent(&mut self, a: usize) -> usize {
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.parent(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.parent(a);
        -self.parent_or_size[x] as usize
    }

    pub fn make_groups(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            parent_buf[i] = self.parent(i);
            group_size[parent_buf[i]] += 1;
        }
        let mut res = vec![Vec::new(); self.n];
        for i in 0..self.n {
            res[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            res[parent_buf[i]].push(i);
        }
        res.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
