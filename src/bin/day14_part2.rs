use std::{
    io::{stdin, BufRead},
    ops::{Add, Mul},
};

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: (self.x + rhs.x).rem_euclid(101),
            y: (self.y + rhs.y).rem_euclid(103),
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: (self.x * rhs).rem_euclid(101),
            y: (self.y * rhs).rem_euclid(103),
        }
    }
}

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|s| s.unwrap()).collect();
    let robots: Vec<(Point, Point)> = input
        .into_iter()
        .map(|line| {
            let (p_in, v_in) = line.split_once(" ").unwrap();
            let (px, py) = p_in.strip_prefix("p=").unwrap().split_once(",").unwrap();
            let (vx, vy) = v_in.strip_prefix("v=").unwrap().split_once(",").unwrap();
            let p = Point {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
            };
            let v = Point {
                x: vx.parse().unwrap(),
                y: vy.parse().unwrap(),
            };
            (p, v)
        })
        .collect();

    // print out every grid, output to file, then text search for
    // a bunch of consecutive #s until you find the tree, lol
    for i in 0..10000 {
        let mut grid: Vec<Vec<u8>> = vec![vec![b'.'; 101]; 103];
        for &(p, v) in &robots {
            let pos = p + (v * i);
            grid[pos.y as usize][pos.x as usize] = b'#';
        }
        println!("iteration {}", i);
        for line in grid {
            println!("{}", String::from_utf8(line).unwrap());
        }
    }
}
