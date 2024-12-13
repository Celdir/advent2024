use std::io::{stdin, Read};

struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let mut input = String::new();
    stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("failed to read input");

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
            x: 10000000000000 + px.strip_prefix("X=").unwrap().parse::<isize>().unwrap(),
            y: 10000000000000 + py.strip_prefix("Y=").unwrap().parse::<isize>().unwrap(),
        };

        /*
         * [ ax bx ] [ c ] = [ px ]
         * [ ay by ] [ d ]   [ py ]
         *
         */
        let det = a.x * b.y - a.y * b.x;
        let cdet = b.y * prize.x - b.x * prize.y;
        let ddet = a.x * prize.y - a.y * prize.x;
        if cdet % det != 0 || ddet % det != 0 {
            continue;
        }
        let c = cdet / det;
        let d = ddet / det;
        let cost = 3 * c + d;
        ans += cost;
    }
    println!("{}", ans);
}
