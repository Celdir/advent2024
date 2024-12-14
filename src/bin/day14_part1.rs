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
    let mut quads = [0; 4];
    for line in input {
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

        let pos = p + (v * 100);
        if pos.y == 51 || pos.x == 50 {
            continue;
        }
        let quad = ((pos.y / 52) * 2 + (pos.x / 51)) as usize;
        quads[quad] += 1;
    }
    let ans = quads.iter().fold(1, |sum, i| sum * i);
    println!("{}", ans);
}
