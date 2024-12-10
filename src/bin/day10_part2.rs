use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use cgraph::{
    graph::{
        grid::{Grid, Position},
        traits::Graph,
    },
    iter::bfs::bfs,
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
                let visited: Vec<_> = bfs(&graph, (i, j).into()).map(|(_, node)| node).collect();
                let mut memo: HashMap<Position, usize> = HashMap::new();
                for node in visited.iter().rev() {
                    let (i, j) = node.id().into();
                    if input[i as usize][j as usize] == b'9' {
                        memo.insert(node.id(), 1);
                        continue;
                    }
                    let mut paths = 0;
                    for (_, adj) in graph.adj(node.id()).unwrap() {
                        if let Some(v) = memo.get(&adj.id()) {
                            paths += v;
                        }
                    }
                    memo.insert(node.id(), paths);
                }
                ans += memo[&(i, j).into()];
            }
        }
    }
    println!("{}", ans);
}
