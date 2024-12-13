use std::{
    collections::HashMap,
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

    let neighbors = |i: usize, j: usize| {
        let ii = i as isize;
        let jj = j as isize;
        let mut adj = Vec::new();
        for (di, dj) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (ni, nj) = (ii + di, jj + dj);
            if (0..n).contains(&ni)
                && (0..m).contains(&nj)
                && input[i][j] == input[ni as usize][nj as usize]
            {
                adj.push((ni as usize, nj as usize));
            }
        }
        adj
    };

    let mut component: HashMap<(usize, usize), usize> = HashMap::new();
    let mut current_component = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if component.contains_key(&(i, j)) {
                continue;
            }
            component.insert((i, j), current_component);
            current_component += 1;
            let mut bfs = neighbors(i, j);
            while !bfs.is_empty() {
                let n = bfs.pop().unwrap();
                if !component.contains_key(&n) {
                    component.insert(n, component[&(i, j)]);
                    bfs.append(&mut neighbors(n.0, n.1));
                }
            }
        }
    }

    let mut area = vec![0; current_component];
    let mut shared_edges = vec![0; current_component];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let c = component[&(i, j)];
            area[c] += 1;
            shared_edges[c] += neighbors(i, j).into_iter().count();
        }
    }
    let mut ans = 0;
    for c in 0..area.len() {
        let perimeter = 4 * area[c] - shared_edges[c];
        let price = area[c] * perimeter;
        ans += price;
    }
    println!("{}", ans);
}
