use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

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

fn pathdist(
    startdist: &HashMap<State, usize>,
    enddist: &HashMap<Direction, HashMap<State, usize>>,
    state: State,
) -> Option<usize> {
    // shortest path going through state {pos, dir}
    // is dist(start_state, state) + min(dist(end_state, {pos, dir.opposite()})) for each
    // of the 4 directed end states because path distances are symmetrical
    let sdist = startdist.get(&state)?;
    let edist = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .map(|d| {
        enddist[&d].get(&State {
            pos: state.pos,
            direction: state.direction.opposite(),
        })
    })
    .filter_map(|dist| Some(dist?))
    .min()?;
    Some(sdist + edist)
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
    let mut start_opt: Option<State> = None;
    let mut end_opt: Option<Position> = None;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            match input[i][j] {
                b'S' => {
                    start_opt = Some(State {
                        pos: (i as isize, j as isize).into(),
                        direction: Direction::Right,
                    });
                }
                b'E' => {
                    end_opt = Some((i as isize, j as isize).into());
                }
                _ => (),
            };
        }
    }
    let start = start_opt.unwrap();
    let endpos = end_opt.unwrap();

    let mut bestpath = usize::MAX;
    let startdist: HashMap<State, usize> = dijkstra(&graph, start)
        .unwrap()
        .map(|(_, node, dist)| (node.id(), dist))
        .collect();
    let mut enddist: HashMap<Direction, HashMap<State, usize>> = HashMap::new();
    for dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        bestpath = min(
            bestpath,
            startdist[&State {
                pos: endpos,
                direction: dir,
            }],
        );
        enddist.insert(
            dir,
            dijkstra(
                &graph,
                State {
                    pos: endpos,
                    direction: dir,
                },
            )
            .unwrap()
            .map(|(_, node, dist)| (node.id(), dist))
            .collect(),
        );
    }

    let mut tiles = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let state = State {
                    pos: (i, j).into(),
                    direction: dir,
                };
                if let Some(dist) = pathdist(&startdist, &enddist, state) {
                    if dist == bestpath {
                        tiles.insert(state.pos);
                    }
                }
            }
        }
    }

    println!("{}", tiles.len());
}
