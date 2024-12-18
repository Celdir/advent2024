use std::io::{stdin, BufRead};

use cgraph::{
    graph::grid::{Grid, Position},
    iter::{bfs::bfs, traits::Traversal},
};

fn main() {
    let bytes: Vec<(isize, isize)> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (a.to_owned(), b.to_owned())
        })
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let n: isize = 71;
    let m: isize = 71;
    let mut blocked: Vec<isize> = vec![-1; (n * m) as usize];
    let index = |i: isize, j: isize| (i * m + j) as usize;
    for (idx, &(i, j)) in bytes.iter().enumerate() {
        blocked[index(i, j)] = idx as isize;
    }
    let graph = Grid::four_connected(
        |_| {},
        |_: Position, _: Position| 1,
        |pos: Position| {
            let (i, j) = pos.into();
            (0..n).contains(&i) && (0..m).contains(&j)
        },
        |_: Position, v: Position| {
            let (vi, vj) = v.into();
            (0..n).contains(&vi)
                && (0..m).contains(&vj)
                && (blocked[index(vi, vj)] == -1 || blocked[index(vi, vj)] >= 1024)
        },
    );
    let start = (0, 0).into();
    let end = (70, 70).into();
    let ans = bfs(&graph, start)
        .find_path_to(end)
        .unwrap()
        .edges()
        .count();
    println!("{}", ans);
}
