use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

use cgraph::{
    algo::shortest_paths::dijkstra::dijkstra,
    graph::{
        grid::{Grid, Position},
        traits::Graph,
    },
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
        |_, _| 1,
        node_filter,
        |u, v| {
            if !node_filter(u) || !node_filter(v) {
                return false;
            }
            let (vi, vj) = v.into();
            input[vi as usize][vj as usize] != b'#'
        },
    );
    let mut start: Option<Position> = None;
    let mut end: Option<Position> = None;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            match input[i][j] {
                b'S' => start = Some((i as isize, j as isize).into()),
                b'E' => end = Some((i as isize, j as isize).into()),
                _ => (),
            };
        }
    }
    let startdist: HashMap<_, _> = dijkstra(&graph, start.unwrap())
        .unwrap()
        .map(|(_, node, dist)| (node.id(), dist))
        .collect();
    let enddist: HashMap<_, _> = dijkstra(&graph, end.unwrap())
        .unwrap()
        .map(|(_, node, dist)| (node.id(), dist))
        .collect();
    let besttime = startdist[&end.unwrap()];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] == b'#' {
                let neighbors: Vec<_> = graph.adj((i, j).into()).unwrap().collect();
                let beststart = neighbors
                    .iter()
                    .filter_map(|(_, node)| startdist.get(&node.id()))
                    .min();
                let bestend = neighbors
                    .iter()
                    .filter_map(|(_, node)| enddist.get(&node.id()))
                    .min();
                if let Some(bs) = beststart {
                    if let Some(be) = bestend {
                        let dist = bs + be + 2;
                        if dist + 100 <= besttime {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
