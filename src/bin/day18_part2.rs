use std::io::{stdin, BufRead};

use cgraph::{
    graph::{
        grid::{Direction, Position},
        state::StateGraph,
    },
    iter::bfs::bfs,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct State {
    pos: Position,
}

fn main() {
    let bytes: Vec<(usize, usize)> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (a, b) = l.split_once(",").unwrap();
            (a.to_owned(), b.to_owned())
        })
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let n = 71;
    let m = 71;
    let mut grid = vec![vec![b'.'; m as usize]; n as usize];
    for (i, j) in bytes {
        grid[i][j] = b'#';
        let graph = StateGraph::new(
            |_| {},
            |_: State, _: State| 1,
            |state: State| {
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .into_iter()
                .map(|dir| state.pos + dir)
                .filter(|&pos| {
                    let (ni, nj) = pos.into();
                    (0..n).contains(&ni) && (0..m).contains(&nj)
                })
                .filter(|&pos| {
                    let (ni, nj) = pos.into();
                    match grid[ni as usize][nj as usize] {
                        b'#' => false,
                        _ => true,
                    }
                })
                .map(|pos| State { pos })
                .collect()
            },
            |state: State| {
                let (i, j) = state.pos.into();
                (0..n).contains(&i) && (0..m).contains(&j)
            },
        );
        let start = State { pos: (0, 0).into() };
        let end = State {
            pos: (70, 70).into(),
        };
        let mut bfs = bfs(&graph, start);
        if bfs.find(|(_, node)| node.id() == end).is_none() {
            println!("{},{}", i, j);
            break;
        }
    }
}
