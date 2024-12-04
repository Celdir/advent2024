use std::io::{stdin, BufRead};

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect();
    let n = input.len() as isize;
    let m = input[0].len() as isize;

    let mut deltas: Vec<(isize, isize)> = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }
            deltas.push((di, dj));
        }
    }

    let xmas = "XMAS".as_bytes();
    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            for delta in &deltas {
                let mut pos = vec![(i, j)];
                for idx in 1..xmas.len() {
                    let p = (pos[idx - 1].0 + delta.0, pos[idx - 1].1 + delta.1);
                    pos.push(p);
                }
                let mut works = true;
                for idx in 0..xmas.len() {
                    let p = pos[idx];
                    if !(0..n).contains(&p.0)
                        || !(0..m).contains(&p.1)
                        || xmas[idx] != input[p.0 as usize][p.1 as usize]
                    {
                        works = false;
                        break;
                    }
                }
                if works {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
