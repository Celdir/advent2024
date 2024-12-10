use std::io::{stdin, BufRead};

use cgraph::{
    graph::grid::{Grid, Position},
    iter::dfs::dfs,
};

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();
    let n = input.len() as isize;
    let m = input[0].len() as isize;
    let node_filter = |pos: Position| {
        let (i, j) = pos.into();
        (0..n).contains(&i) && (0..m).contains(&j)
    };
    let graph = Grid::four_connected(
        |pos: Position| {
            let (i, j) = pos.into();
            input[i as usize][j as usize]
        },
        |_, _| {},
        node_filter,
        |u, v| {
            if !node_filter(u) || !node_filter(v) {
                return false;
            }
            let (ui, uj) = u.into();
            let (vi, vj) = v.into();
            let uval = input[ui as usize][uj as usize];
            let vval = input[vi as usize][vj as usize];
            return vval == uval + 1;
        },
    );
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] == b'0' {
                let count = dfs(&graph, (i, j).into())
                    .filter(|&(_, node)| *node == b'9')
                    .count();
                ans += count;
            }
        }
    }
    println!("{}", ans);
}
