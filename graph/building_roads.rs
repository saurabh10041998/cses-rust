#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A string");
        let $out = inner.trim().parse::<$type>().expect("A parsable");
    };
}
#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A string");
        let $out = inner.trim().chars().collect::<Vec<char>>();
    };
}
#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A string");
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let rank = vec![1; n];
        for i in 0..n {
            parent[i] = i;
        }
        DisjointSet { parent, rank }
    }

    fn find_parent(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            return self.parent[idx];
        }
        self.parent[idx] = self.find_parent(self.parent[idx]);
        self.parent[idx]
    }

    fn union_by_size(&mut self, x: usize, y: usize) -> bool {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);

        if par_x == par_y {
            return false;
        }

        if self.rank[par_x] < self.rank[par_y] {
            self.rank[par_y] += self.rank[par_x];
            self.parent[par_x] = par_y;
        } else {
            self.rank[par_x] += self.rank[par_y];
            self.parent[par_y] = par_x;
        }
        true
    }
}

fn solve() {
    read_vec!(arr as i32);
    let n = arr[0] as usize;
    let m = arr[1] as usize;

    let mut ds = DisjointSet::new(n);

    for _ in 0..m {
        read_vec!(edge as usize);
        let u = edge[0] - 1;
        let v = edge[1] - 1;

        if ds.find_parent(u) != ds.find_parent(v) {
            ds.union_by_size(u, v);
        }
    }

    let mut cnt = 0;

    let mut parents = vec![];

    for i in 0..ds.parent.len() {
        if i == ds.parent[i] {
            cnt += 1;
            parents.push(i + 1);
        }
    }

    println!("{}", cnt - 1);

    for i in 0..(cnt - 1) as usize {
        println!("{} {}", parents[i], parents[i + 1]);
    }
}

fn main() {
    solve()
}
