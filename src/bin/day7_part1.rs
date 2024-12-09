use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let mut ans = 0;
    for line in input {
        let (result_str, operands_str) = line.split_once(":").unwrap();
        let result: usize = result_str.parse().unwrap();
        let operands: Vec<usize> = operands_str
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let ops = operands.len() - 1;
        for mask in 0..(1 << ops) {
            let mut val = operands[0];
            for i in 0..ops {
                if mask & (1 << i) > 0 {
                    val *= operands[i + 1];
                } else {
                    val += operands[i + 1];
                }
                if val > result {
                    break;
                }
            }
            if val == result {
                ans += result;
                break;
            }
        }
    }
    println!("{}", ans);
}
