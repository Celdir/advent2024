use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

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

fn gen(cur: u8, c: u8) -> Vec<u8> {
    let pos = |c| match c {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("unexpected char"),
    };
    let mut gen = vec![];
    let u: (i32, i32) = pos(cur);
    let v = pos(c);
    let delta = (v.0 - u.0, v.1 - u.1);
    let vertical = match delta.0 {
        i if i < 0 => b'^',
        _ => b'v',
    };
    let horizontal = match delta.1 {
        i if i < 0 => b'<',
        _ => b'>',
    };
    let fastorder = [b'<', b'v', b'^', b'>'];
    let safeorder = [b'>', b'v', b'^', b'<'];
    let order = match c {
        b'<' => safeorder,
        _ => match cur {
            b'<' => safeorder,
            _ => fastorder,
        },
    };
    for dir in order {
        if vertical == dir {
            for _ in 0..delta.0.abs() {
                gen.push(vertical);
            }
        }
        if horizontal == dir {
            for _ in 0..delta.1.abs() {
                gen.push(horizontal);
            }
        }
    }
    gen.push(b'A');
    gen
}

fn robot(a: u8, b: u8, iter: isize, memo: &mut HashMap<(u8, u8, isize), usize>) -> usize {
    if memo.contains_key(&(a, b, iter)) {
        return memo[&(a, b, iter)];
    }
    if iter == 0 {
        return 1;
    }
    let g = gen(a, b);
    let mut cur = b'A';
    let mut ans = 0;
    for c in g {
        ans += robot(cur, c, iter - 1, memo);
        cur = c;
    }
    memo.insert((a, b, iter), ans);
    ans
}

fn main() {
    let codes: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut ans = 0;
    for code in codes {
        let numeric: usize = code[0..3].parse().unwrap();
        let path = door(code);
        let mut memo = HashMap::new();
        let mut cur = b'A';
        let mut len = 0;
        for &c in path.as_bytes() {
            len += robot(cur, c, 25, &mut memo);
            cur = c;
        }
        ans += numeric * len;
    }
    println!("{}", ans);
}
