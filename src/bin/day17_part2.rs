use std::{
    cmp::min,
    io::{stdin, BufRead},
};

// input program does this:
// b = a % 8
// b = b XOR 3
// c = a / 2 ^ b = a >> b
// b = b XOR 5
// a = a / 8
// b = b XOR c
// output b % 8
// if a != 0: goto beginning

fn dfs(program: &Vec<usize>, idx: usize, a: usize) -> Option<usize> {
    let mut ans = None;
    for apart in 0..8 {
        let aprime = a * 8 + apart;
        let mut b = apart ^ 3;
        let c = aprime >> b;
        b = (b ^ 5 ^ c) % 8;
        if b == program[idx] {
            let val = match idx {
                0 => Some(aprime),
                _ => dfs(&program, idx - 1, aprime),
            };
            ans = match ans {
                None => val,
                Some(a) => match val {
                    Some(v) => Some(min(a, v)),
                    None => ans,
                },
            };
        }
    }
    ans
}

fn main() {
    let input: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();
    let program: Vec<usize> = input[3]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let ans = dfs(&program, program.len() - 1, 0);
    println!("{}", ans.unwrap());
}
