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
        let $out = inner.trim();
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

fn dfs(row: i32, col: i32, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    visited[row as usize][col as usize] = true;
    let delrow = vec![-1, 0, 1, 0];
    let delcol = vec![0, 1, 0, -1];

    for i in 0..4 {
        let nrow = row + delrow[i];
        let ncol = col + delcol[i];

        if nrow >= 0
            && nrow < n
            && ncol >= 0
            && ncol < m
            && grid[nrow as usize][ncol as usize] == '.'
            && visited[nrow as usize][ncol as usize] == false
        {
            dfs(nrow, ncol, grid, visited);
        }
    }
}

fn solve() {
    read_vec!(arr as usize);
    let (n, m) = (arr[0], arr[1]);

    let mut grid = vec![];
    for _ in 0..n {
        read_str!(s);
        grid.push(s.chars().collect::<Vec<_>>());
    }

    let mut visited = vec![vec![false; m]; n];

    let mut cnt = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '.' && visited[i][j] == false {
                cnt += 1;
                dfs(i as i32, j as i32, &grid, &mut visited);
            }
        }
    }
    println!("{}", cnt);
}

fn main() {
    solve();
}
