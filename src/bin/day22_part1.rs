use std::io::{stdin, BufRead};

fn evolve(mut secret: u64) -> u64 {
    let base = 16777216;
    secret = (secret ^ (secret * 64)) % base;
    secret = (secret ^ (secret / 32)) % base;
    secret = (secret ^ (secret * 2048)) % base;
    secret
}

fn main() {
    let seeds: Vec<u64> = stdin().lock().lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut ans = 0;
    for seed in seeds {
        let mut secret = seed;
        for _ in 0..2000 {
            secret = evolve(secret);
        }
        ans += secret;
    }
    println!("{}", ans);
}
