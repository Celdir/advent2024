use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};

use cgraph::{
    graph::{
        grid::{Direction, Position},
        state::StateGraph,
        traits::Graph,
    },
    iter::dfs::dfs,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct State {
    pos: Position,
    direction: Direction,
}

fn build_graph(input: &Vec<Vec<u8>>) -> impl Graph<NId = State> + use<'_> {
    let n = input.len() as isize;
    let m = input[0].len() as isize;
    let filter = move |state: State| {
        let (i, j) = state.pos.into();
        (0..n).contains(&i) && (0..m).contains(&j)
    };
    let transition = move |state: State| {
        let adj = state.pos + state.direction;
        let (ni, nj) = adj.into();
        if !(0..n).contains(&ni) || !(0..m).contains(&nj) {
            return Vec::new();
        }
        let next = match input[ni as usize][nj as usize] {
            b'#' => {
                let ndir = match state.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    _ => panic!("should never encounter non-cardinal direction"),
                };
                State {
                    pos: state.pos,
                    direction: ndir,
                }
            }
            _ => State {
                pos: adj,
                direction: state.direction,
            },
        };
        vec![next]
    };
    StateGraph::new(|_| {}, |_, _| {}, transition, filter)
}

fn is_cycle(input: &Vec<Vec<u8>>, start: State) -> bool {
    let graph = build_graph(input);
    let last = dfs(&graph, start).last().unwrap().1.id();
    let next = graph.adj(last).unwrap().next();
    next.is_some()
}

fn main() {
    let mut input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();
    let mut start = None;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != b'#' && input[i][j] != b'.' {
                start = Some(State {
                    pos: (i as isize, j as isize).into(),
                    direction: match input[i][j] {
                        b'<' => Direction::Left,
                        b'>' => Direction::Right,
                        b'^' => Direction::Up,
                        b'v' => Direction::Down,
                        _ => panic!("should never encounter other char"),
                    },
                });
            }
        }
    }

    let visited: HashSet<Position> = dfs(&build_graph(&input), start.unwrap())
        .map(|(_, node)| node.id().pos)
        .collect();

    let mut count = 0;
    for p in visited {
        let (ii, jj) = p.into();
        let (i, j) = (ii as usize, jj as usize);
        let val = input[i][j];
        if val != b'.' {
            continue;
        }
        input[i][j] = b'#';
        if is_cycle(&input, start.unwrap()) {
            count += 1;
        }
        input[i][j] = b'.';
    }

    println!("{}", count);
}
