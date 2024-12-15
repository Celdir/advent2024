use std::io::{stdin, BufRead, Read};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => panic!("invalid state"),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn direction(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    let (i, j) = pos;
    match dir {
        Direction::Up => (i - 1, j),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
    }
}

fn act(grid: &mut Vec<Vec<u8>>, robot: (usize, usize), dir: Direction) -> (usize, usize) {
    let mut next = direction(robot, dir);
    while grid[next.0][next.1] == b'O' {
        next = direction(next, dir);
    }
    match grid[next.0][next.1] {
        b'#' => robot,
        b'.' => {
            while next != robot {
                let adj = direction(next, dir.opposite());
                let tmp = grid[next.0][next.1];
                grid[next.0][next.1] = grid[adj.0][adj.1];
                grid[adj.0][adj.1] = tmp;
                next = adj;
            }
            direction(robot, dir)
        }
        _ => panic!("impossible state"),
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let (grid_str, moves_str) = input.split_once("\n\n").unwrap();
    let moves = moves_str.replace("\n", "").as_bytes().to_vec();
    let mut grid: Vec<_> = grid_str
        .split("\n")
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let mut robot_opt = None;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'@' {
                robot_opt = Some((i, j));
                grid[i][j] = b'.';
            }
        }
    }
    let mut robot = robot_opt.unwrap();
    for mv in moves {
        robot = act(&mut grid, robot, mv.into());
    }


    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'O' {
                ans += 100 * i + j;
            }
        }
    }
    println!("{}", ans);
}
