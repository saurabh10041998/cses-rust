use std::collections::HashMap;

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

fn dfs(
    src: usize,
    parent: i32,
    adj: &HashMap<usize, Vec<usize>>,
    visited: &mut Vec<bool>,
    path: &mut Vec<i32>,
) -> bool {
    visited[src] = true;
    path.push(src as i32);

    match adj.get(&src) {
        Some(neighbors) => {
            for &nei in neighbors {
                if visited[nei] == false {
                    if dfs(nei, src as i32, adj, visited, path) {
                        return true;
                    }
                } else {
                    if nei != parent as usize {
                        path.push(nei as i32);
                        return true;
                    }
                }
            }
        }
        None => {}
    }
    path.pop().unwrap();
    false
}

fn solve() {
    read_vec!(arr as i32);
    let (n, m) = (arr[0], arr[1]);

    let mut adj = HashMap::new();
    for _ in 0..m {
        read_vec!(edge as usize);
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        adj.entry(u)
            .and_modify(|buf: &mut Vec<usize>| buf.push(v))
            .or_insert(vec![v]);
        adj.entry(v)
            .and_modify(|buf: &mut Vec<usize>| buf.push(u))
            .or_insert(vec![u]);
    }

    let mut visited = vec![false; n as usize];

    for i in 0..n as usize {
        if visited[i] == false {
            let mut path = vec![];
            if dfs(i, -1, &adj, &mut visited, &mut path) {
                let mut cycle = vec![];
                let mut ptr = path.len() - 1;
                let seed = path[ptr];

                cycle.push(seed);
                ptr -= 1;

                while path[ptr] != seed {
                    cycle.push(path[ptr]);
                    ptr -= 1;
                }

                cycle.push(seed);

                println!("{}", cycle.len());

                for x in cycle.into_iter().rev() {
                    print!("{} ", x + 1);
                }
                println!();

                return;
            }
        }
    }
    println!("IMPOSSIBLE");
}

fn main() {
    solve();
}
