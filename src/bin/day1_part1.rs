use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in &input {
        let parts: Vec<i32> = line
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(parts.len(), 2);
        left.push(parts[0]);
        right.push(parts[1]);
    }
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    println!("{}", sum);
}
