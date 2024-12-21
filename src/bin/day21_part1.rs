use std::io::{stdin, BufRead};

fn door(code: String) -> String {
    let pos = |c| match c {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => panic!("unexpected char"),
    };
    let mut cur = 'A';
    let mut ans = String::new();
    for c in code.chars() {
        let u: (i32, i32) = pos(cur);
        let v = pos(c);
        let delta = (v.0 - u.0, v.1 - u.1);
        let vertical = match delta.0 {
            i if i < 0 => '^',
            _ => 'v',
        };
        let horizontal = match delta.1 {
            i if i < 0 => '<',
            _ => '>',
        };
        let fastorder = ['<', 'v', '^', '>'];
        let safeorder = ['^', '>', 'v', '<'];
        let order = match c {
            '7' | '4' | '1' => match cur {
                '0' | 'A' => safeorder,
                _ => fastorder,
            },
            '0' | 'A' => match cur {
                '7' | '4' | '1' => safeorder,
                _ => fastorder,
            },
            _ => fastorder,
        };
        for dir in order {
            if vertical == dir {
                for _ in 0..delta.0.abs() {
                    ans.push(vertical);
                }
            }
            if horizontal == dir {
                for _ in 0..delta.1.abs() {
                    ans.push(horizontal);
                }
            }
        }
        ans.push('A');
        cur = c;
    }
    ans
}

fn robot(code: String) -> String {
    let pos = |c| match c {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => panic!("unexpected char"),
    };
    let mut cur = 'A';
    let mut ans = String::new();
    for c in code.chars() {
        let u: (i32, i32) = pos(cur);
        let v = pos(c);
        let delta = (v.0 - u.0, v.1 - u.1);
        let vertical = match delta.0 {
            i if i < 0 => '^',
            _ => 'v',
        };
        let horizontal = match delta.1 {
            i if i < 0 => '<',
            _ => '>',
        };
        for dir in ['>', '^', 'v', '<'] {
            if vertical == dir {
                for _ in 0..delta.0.abs() {
                    ans.push(vertical);
                }
            }
            if horizontal == dir {
                for _ in 0..delta.1.abs() {
                    ans.push(horizontal);
                }
            }
        }
        ans.push('A');
        cur = c;
    }
    ans
}

fn main() {
    let codes: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut ans = 0;
    for code in codes {
        let numeric: usize = code[0..3].parse().unwrap();
        let path = robot(robot(door(code.clone())));
        ans += numeric * path.len();
    }
    println!("{}", ans);
}
