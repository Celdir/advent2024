use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect();
    let n = input.len() as isize;
    let m = input[0].len() as isize;

    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i as usize][j as usize] != b'A' {
                continue;
            }
            let mut works = true;
            let mut ms = Vec::new();
            let mut ss = Vec::new();
            for di in &[-1, 1] {
                for dj in &[-1, 1] {
                    let (ni, nj) = (i + di, j + dj);
                    if !(0..n).contains(&ni) || !(0..m).contains(&nj) {
                        works = false;
                        break;
                    }
                    let val = input[ni as usize][nj as usize];
                    match val {
                        b'M' => {
                            ms.push((ni, nj));
                        }
                        b'S' => {
                            ss.push((ni, nj));
                        }
                        _ => {
                            works = false;
                            break;
                        }
                    };
                }
            }
            if ms.len() != 2 || ss.len() != 2 {
                works = false;
            } else {
                if ms[0].0 != ms[1].0 && ms[0].1 != ms[1].1 {
                    works = false;
                }
                if ss[0].0 != ss[1].0 && ss[0].1 != ss[1].1 {
                    works = false;
                }
            }

            if works {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
