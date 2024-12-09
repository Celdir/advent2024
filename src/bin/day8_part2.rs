use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() {
    let input: Vec<Vec<u8>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().bytes().collect())
        .collect();
    let n = input.len() as isize;
    let m = input[0].len() as isize;

    let mut antenna: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] != b'.' {
                antenna
                    .entry(input[i][j])
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (_, loc) in antenna {
        for i in 0..loc.len() {
            let a = loc[i];
            for j in 0..loc.len() {
                if i == j {
                    continue;
                }
                let b = loc[j];
                let (di, dj) = (b.0 - a.0, b.1 - a.1);
                let mut antinode = b;
                while (0..n).contains(&antinode.0) && (0..m).contains(&antinode.1) {
                    antinodes.insert(antinode);
                    antinode.0 += di;
                    antinode.1 += dj;
                }
            }
        }
    }
    println!("{}", antinodes.len());
}
