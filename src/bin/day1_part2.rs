use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut left = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in &input {
        let parts: Vec<i32> = line
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(parts.len(), 2);
        left.push(parts[0]);
        *right.entry(parts[1]).or_default() += 1;
    }

    let mut sim = 0;
    for i in &left {
        sim += i * right.get(i).unwrap_or(&0);
    }

    println!("{}", sim);
}
