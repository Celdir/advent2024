use std::{
    cmp::min,
    io::{stdin, Read},
    ops::{Add, Mul, Rem, Sub},
};

#[derive(Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Rem<Point> for Point {
    type Output = Point;
    fn rem(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("failed to read input");
    let zero = Point { x: 0, y: 0 };

    let mut ans = 0;
    for game in input.split("\n\n") {
        let lines: Vec<_> = game.split("\n").filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 3);
        let (ax, ay) = lines[0]
            .strip_prefix("Button A: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let a = Point {
            x: ax.strip_prefix("X+").unwrap().parse().unwrap(),
            y: ay.strip_prefix("Y+").unwrap().parse().unwrap(),
        };
        let (bx, by) = lines[1]
            .strip_prefix("Button B: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let b = Point {
            x: bx.strip_prefix("X+").unwrap().parse().unwrap(),
            y: by.strip_prefix("Y+").unwrap().parse().unwrap(),
        };
        let (px, py) = lines[2]
            .strip_prefix("Prize: ")
            .unwrap()
            .split_once(", ")
            .unwrap();
        let prize = Point {
            x: px.strip_prefix("X=").unwrap().parse().unwrap(),
            y: py.strip_prefix("Y=").unwrap().parse().unwrap(),
        };

        let mut cost = None;
        let mut c = 0;
        while (a * c) < prize {
            let r = prize - (a * c);
            if r % b == zero {
                let (dx, dy) = (r.x / b.x, r.y / b.y);
                if dx == dy {
                    let tokens = 3 * c + dx;
                    cost = match cost {
                        Some(cst) => Some(min(cst, tokens)),
                        _ => Some(tokens),
                    };
                }
            }
            c += 1;
        }
        ans += cost.unwrap_or(0);
    }
    println!("{}", ans);
}
