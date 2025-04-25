use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

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

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    dst: usize,
    weight: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    node: usize,
    distance: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.distance == other.distance {
            return self.node.partial_cmp(&other.node);
        }
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.distance == other.distance {
            return self.node.cmp(&other.node);
        }
        self.distance.cmp(&other.distance)
    }
}

fn spf(n: usize, adj: &HashMap<usize, Vec<Edge>>) {
    let mut pq = BinaryHeap::new();
    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    pq.push(Reverse(Entry {
        node: 0,
        distance: 0,
    }));

    while let Some(Reverse(e)) = pq.pop() {
        let dis = e.distance;
        let node = e.node;

        match adj.get(&node) {
            Some(neighbors) => {
                for e in neighbors.iter() {
                    let adj_node = e.dst;
                    let edge_weight = e.weight;

                    if dis + edge_weight < dist[adj_node] {
                        dist[adj_node] = dis + edge_weight;
                        pq.push(Reverse(Entry {
                            node: adj_node,
                            distance: dist[adj_node],
                        }))
                    }
                }
            }
            None => {}
        }
    }

    for d in dist {
        print!("{} ", d);
    }
    println!();
}

fn solve() {
    read_vec!(arr as usize);
    let n = arr[0];
    let m = arr[1];

    let mut adj = HashMap::new();
    for _ in 0..m {
        read_vec!(edge as usize);
        let (u, v, wt) = (edge[0] - 1, edge[1] - 1, edge[2]);
        adj.entry(u)
            .and_modify(|buf: &mut Vec<Edge>| buf.push(Edge { dst: v, weight: wt }))
            .or_insert(vec![Edge { dst: v, weight: wt }]);
    }

    spf(n, &adj);
}

fn main() {
    solve();
}
