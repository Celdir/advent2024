use std::io::{stdin, BufRead};

struct TernaryMask(Vec<u8>);
impl TernaryMask {
    fn increment(&mut self) -> bool {
        for i in 0..self.0.len() {
            self.0[i] += 1;
            if self.0[i] <= 2 {
                return true;
            }
            self.0[i] = 0;
        }
        return false;
    }
}

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
        let mut tmask = TernaryMask(vec![0; ops]);
        let mut go = true;
        while go {
            let mut val = operands[0];
            for i in 0..ops {
                match tmask.0[i] {
                    0 => {
                        val += operands[i + 1];
                    }
                    1 => {
                        val *= operands[i + 1];
                    }
                    2 => {
                        val = (val.to_string() + &operands[i + 1].to_string())
                            .parse()
                            .unwrap();
                    }
                    _ => panic!("impossible state"),
                }
                if val > result {
                    break;
                }
            }
            if val == result {
                ans += result;
                break;
            }
            go = tmask.increment();
        }
    }
    println!("{}", ans);
}
