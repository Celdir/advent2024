use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut count = 0;
    for line in &input {
        let report: Vec<i32> = line.split(" ").map(|i| i.parse().unwrap()).collect();
        let mut safe = true;
        let mut last_diff = report[1] - report[0];
        if last_diff.abs() < 1 || last_diff.abs() > 3 {
            safe = false;
        }
        for i in 1..report.len() - 1 {
            let diff = report[i + 1] - report[i];
            if diff.abs() < 1 || diff.abs() > 3 || last_diff.is_positive() != diff.is_positive() {
                safe = false;
            }
            last_diff = diff;
        }
        if safe {
            count += 1;
        }
    }
    println!("{}", count)
}
