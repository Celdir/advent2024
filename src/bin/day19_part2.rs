use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn possible<'a>(
    memo: &mut HashMap<&'a [u8], usize>,
    patterns: &Vec<Vec<u8>>,
    towel: &'a [u8],
) -> usize {
    if towel.is_empty() {
        return 1;
    }
    if let Some(&ans) = memo.get(&towel) {
        return ans;
    }
    let mut ans = 0;
    for p in patterns {
        if towel.starts_with(p) {
            ans += possible(memo, patterns, &towel[p.len()..]);
        }
    }
    memo.insert(towel, ans);
    ans
}

fn main() {
    let input: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();
    let patterns: Vec<Vec<u8>> = input[0]
        .split(", ")
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let towels: Vec<Vec<u8>> = input[1..].iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut ans = 0;
    let mut memo = HashMap::new();
    for towel in &towels {
        ans += possible(&mut memo, &patterns, &towel[..]);
    }
    println!("{}", ans);
}
