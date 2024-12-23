use std::{io::{stdin, BufRead}, collections::HashMap};

fn evolve(mut secret: i64) -> i64 {
    let base = 16777216;
    secret = (secret ^ (secret * 64)) % base;
    secret = (secret ^ (secret / 32)) % base;
    secret = (secret ^ (secret * 2048)) % base;
    secret
}

fn main() {
    let seeds: Vec<i64> = stdin().lock().lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut total: HashMap<Vec<i64>, i64> = HashMap::new();
    for seed in seeds {
        let mut seq_vals: HashMap<Vec<i64>, i64> = HashMap::new();
        let mut secret = seed;
        let mut last = secret % 10;
        let mut changes = vec![];
        for _ in 0..2000 {
            secret = evolve(secret);
            let cur = secret % 10;
            changes.push(cur - last);
            if changes.len() >= 4 {
                let seq = changes[changes.len()-4..].to_vec();
                if !seq_vals.contains_key(&seq) {
                    seq_vals.insert(seq, cur);
                }
            }
            last = cur;
        }
        for (seq, val) in seq_vals {
            *total.entry(seq).or_default() += val;
        }
    }
    let ans = total.values().max().unwrap();
    println!("{}", ans);
}
