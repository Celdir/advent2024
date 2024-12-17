use std::io::{stdin, BufRead};

use cgraph::{
    algo::shortest_paths::dijkstra::dijkstra,
    graph::{
        grid::{Direction, Position},
        state::StateGraph,
    },
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct State {
    pos: Position,
    direction: Direction,
}

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();
    let n = input.len() as isize;
    let m = input[0].len() as isize;
    let graph = StateGraph::new(
        |_| {},
        |a: State, b: State| {
            if a.direction == b.direction {
                1
            } else {
                1000
            }
        },
        |state: State| {
            let rotations: Vec<Direction> = match state.direction {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
                _ => panic!("impossible state"),
            };
            let mut adj: Vec<State> = rotations
                .into_iter()
                .map(|d| State {
                    pos: state.pos,
                    direction: d,
                })
                .collect();
            let next = state.pos + state.direction;
            let (ni, nj) = next.into();
            if !(0..n).contains(&ni) || !(0..m).contains(&nj) {
                return adj;
            }
            match input[ni as usize][nj as usize] {
                b'#' => (),
                _ => adj.push(State {
                    pos: next,
                    direction: state.direction,
                }),
            };
            adj
        },
        |state: State| {
            let (i, j) = state.pos.into();
            (0..n).contains(&i) && (0..m).contains(&j)
        },
    );
    let mut start: Option<State> = None;
    let mut end: Option<Position> = None;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            match input[i][j] {
                b'S' => {
                    start = Some(State {
                        pos: (i as isize, j as isize).into(),
                        direction: Direction::Right,
                    });
                }
                b'E' => {
                    end = Some((i as isize, j as isize).into());
                }
                _ => (),
            };
        }
    }
    let ans = dijkstra(&graph, start.unwrap())
        .unwrap()
        .filter(|(_, node, _)| node.id().pos == end.unwrap())
        .map(|(_, _, dist)| dist)
        .min()
        .unwrap();
    println!("{}", ans);
}
