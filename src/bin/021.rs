use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    };

    let mut graph = vec![vec![]; n];
    let mut r_graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        r_graph[b - 1].push(a - 1);
    }
    let mut scc = SCC::new(graph, r_graph, n);
    let ans = scc.build();

    println!("{}", ans);
}

#[derive(Debug, Clone, Default)]
pub struct SCC {
    graph: Vec<Vec<usize>>,
    r_graph: Vec<Vec<usize>>,
    vs: Vec<usize>,
    used: Vec<bool>,
    cmp: Vec<usize>,
    size: usize,
    cnt: usize
}

impl SCC {
    pub fn new(graph: Vec<Vec<usize>>, r_graph: Vec<Vec<usize>>, size: usize) -> SCC {
        let used = vec![false; size];
        let cmp = vec![0; size];
        SCC {
            graph: graph,
            r_graph: r_graph,
            used: used,
            size: size,
            cmp: cmp,
            ..Default::default()
        }
    }

    fn dfs(&mut self, v: usize) {
        self.used[v] = true;
        for v in self.graph[v].clone() {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        self.vs.push(v);
    }

    fn r_dfs(&mut self, v: usize) {
        self.used[v] = true;
        self.cmp[v] = k;
        for v in self.r_graph[v].clone() {
            if !self.used[v] {
                self.r_dfs(v);
            }
        }
    }

    pub fn build(&mut self) -> usize {
        self.used.fill(false);
        self.vs.clear();
        for v in 0..self.size {
            if !self.used[v] {
                self.dfs(v);
            }
        }
        let mut ans = 0;
        self.used.fill(false);
        for v in self.vs.clone().iter().rev() {
            if !self.used[*v] {
                self.cnt = 0;
                self.r_dfs(*v);
                ans += 1;
            }
        }
        ans
    }
}
