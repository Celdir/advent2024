use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

enum Type {
    Top,
    Bottom,
    Left,
    Right,
}

impl Type {}

fn count_sides(component: &HashSet<(usize, usize)>, t: Type) -> usize {
    let mut points: Vec<(usize, usize)> = component
        .iter()
        .filter(|&&(i, j)| match t {
            Type::Top => i == 0 || !component.contains(&(i - 1, j)),
            Type::Bottom => !component.contains(&(i + 1, j)),
            Type::Left => j == 0 || !component.contains(&(i, j - 1)),
            Type::Right => !component.contains(&(i, j + 1)),
        })
        .map(|x| x.to_owned())
        .collect();
    points.sort_by_key(|&(i, j)| match t {
        Type::Top | Type::Bottom => (i, j),
        Type::Left | Type::Right => (j, i),
    });
    let mut sides = 1;
    for idx in 1..points.len() {
        let (pi, pj) = points[idx - 1];
        let (i, j) = points[idx];
        match t {
            Type::Top | Type::Bottom => {
                if i != pi || j != pj + 1 {
                    sides += 1;
                }
            }
            Type::Left | Type::Right => {
                if j != pj || i != pi + 1 {
                    sides += 1;
                }
            }
        };
    }
    sides
}

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

    let mut component_id: HashMap<(usize, usize), usize> = HashMap::new();
    let mut components: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut cur_component_id = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if component_id.contains_key(&(i, j)) {
                continue;
            }
            component_id.insert((i, j), cur_component_id);
            cur_component_id += 1;
            components.push(HashSet::from([(i, j)]));
            let mut bfs = neighbors(i, j);
            while !bfs.is_empty() {
                let n = bfs.pop().unwrap();
                if !component_id.contains_key(&n) {
                    let id = component_id[&(i, j)];
                    component_id.insert(n, id);
                    components[id].insert(n);
                    bfs.append(&mut neighbors(n.0, n.1));
                }
            }
        }
    }

    let mut sides: Vec<usize> = vec![0; components.len()];
    for id in 0..components.len() {
        for t in [Type::Top, Type::Bottom, Type::Left, Type::Right] {
            sides[id] += count_sides(&components[id], t);
        }
    }

    let mut ans = 0;
    for id in 0..components.len() {
        let area = components[id].len();
        let price = area * sides[id];
        ans += price;
    }
    println!("{}", ans);
}
