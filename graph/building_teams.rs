use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Color {
    RED,
    GREEN,
    NOCOLOR,
}

fn bfs(src: usize, adj: &HashMap<usize, Vec<usize>>, color: &mut Vec<Color>) -> bool {
    let mut q = VecDeque::new();
    q.push_back(src);
    color[src] = Color::RED;

    while let Some(vertex) = q.pop_front() {
        match adj.get(&vertex) {
            Some(neighbors) => {
                for &v in neighbors {
                    if color[v] == Color::NOCOLOR {
                        color[v] = match color[vertex] {
                            Color::RED => Color::GREEN,
                            Color::GREEN => Color::RED,
                            Color::NOCOLOR => unreachable!("Not possible"),
                        };
                        q.push_back(v);
                    } else if color[v] == color[vertex] {
                        return false;
                    }
                }
            }
            None => {}
        }
    }
    true
}

fn solve() {
    read_vec!(arr as i32);
    let n = arr[0] as usize;
    let m = arr[1] as usize;

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

    let mut color = vec![Color::NOCOLOR; n];
    for i in 0..n {
        if color[i] == Color::NOCOLOR {
            if !bfs(i, &adj, &mut color) {
                println!("IMPOSSIBLE");
                return;
            }
        }
    }

    for i in 0..n {
        match color[i] {
            Color::GREEN => print!("{} ", 1),
            Color::RED => print!("{} ", 2),
            Color::NOCOLOR => unreachable!("Not possible"),
        }
    }
    println!();
}

fn main() {
    solve();
}
