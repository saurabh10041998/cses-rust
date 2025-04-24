// TODO: passes 9/10 testcases
// fails for large size input while printing the path
// buffering is done by the kernel..


use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

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
struct Cell {
    row: usize,
    col: usize,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row, self.col).hash(state)
    }
}

fn bfs(grid: Vec<Vec<char>>, src: Cell, dest: Cell) {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let mut visited: HashSet<Cell> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((src.clone(), String::new()));

    visited.insert(src.clone());

    let delrow = vec![-1, 0, 1, 0];
    let delcol = vec![0, 1, 0, -1];

    while let Some((cell, path)) = q.pop_front() {
        if cell.row == dest.row && cell.col == dest.col {
            println!("YES");
            println!("{}", path.len());
            println!("{}", path);
            return;
        }

        for i in 0..4 {
            let nrow = cell.row as i32 + delrow[i];
            let ncol = cell.col as i32 + delcol[i];

            if nrow >= 0
                && nrow < n
                && ncol >= 0
                && ncol < m
                && grid[nrow as usize][ncol as usize] != '#'
            {
                if !visited.contains(&Cell {
                    row: nrow as usize,
                    col: ncol as usize,
                }) {
                    let direction = match i {
                        0 => 'U',
                        1 => 'R',
                        2 => 'D',
                        3 => 'L',
                        _ => unreachable!(),
                    };
                    let mut new_path = path.clone();
                    new_path.push(direction);
                    q.push_back((
                        Cell {
                            row: nrow as usize,
                            col: ncol as usize,
                        },
                        new_path,
                    ));
                    visited.insert(Cell {
                        row: nrow as usize,
                        col: ncol as usize,
                    });
                }
            }
        }
    }
    println!("NO");
}

fn solve() {
    read_vec!(arr as i32);
    let n = arr[0] as usize;
    let m = arr[1] as usize;
    let mut grid = vec![];

    for _ in 0..n {
        read_str!(s);
        grid.push(s);
    }

    let mut src: Option<Cell> = None;
    let mut dest: Option<Cell> = None;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'A' {
                src = Some(Cell { row: i, col: j });
            }
            if grid[i][j] == 'B' {
                dest = Some(Cell { row: i, col: j });
            }
        }
    }

    match (src, dest) {
        (Some(_src), Some(_dest)) => bfs(grid, _src, _dest),
        (Some(_), None) => unreachable!("Source found, but not destination"),
        (None, Some(_)) => unreachable!("Source not found"),
        (None, None) => unreachable!("Empty grid"),
    };
}

fn main() {
    solve();
}
