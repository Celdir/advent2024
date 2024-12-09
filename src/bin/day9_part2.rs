use std::{
    collections::BTreeMap,
    io::{stdin, BufRead},
};

// 1 + 2 + 3 + ... + n
fn triangle_sum(n: usize) -> usize {
    return n * (n + 1) / 2;
}

fn sum_between(start: usize, end: usize) -> usize {
    let mut ans = triangle_sum(end - 1);
    if start > 0 {
        ans -= triangle_sum(start - 1)
    }
    return ans;
}

fn main() {
    let input: Vec<_> = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .as_bytes()
        .into_iter()
        .map(|b| (b - b'0') as usize)
        .collect();
    let mut file_intervals: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut free_intervals: BTreeMap<usize, usize> = BTreeMap::new();
    let mut block = 0;
    for i in 0..input.len() {
        if i % 2 == 0 {
            file_intervals.insert(block, (block + input[i], i / 2));
        } else {
            free_intervals.insert(block, block + input[i]);
        }
        block += input[i];
    }
    let mut compressed: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (&fstart, &(fend, id)) in file_intervals.iter().rev() {
        let mut found = None;
        for (&start, &end) in &free_intervals {
            if fstart < start {
                break;
            }
            if end - start >= fend - fstart {
                found = Some((start, end));
                break;
            }
        }
        if let Some((start, end)) = found {
            free_intervals.remove(&start);
            compressed.insert(start, (start + (fend - fstart), id));
            if end - start > fend - fstart {
                let diff = (end - start) - (fend - fstart);
                let newstart = start + (fend - fstart);
                free_intervals.insert(newstart, newstart + diff);
            }
        } else {
            compressed.insert(fstart, (fend, id));
        }
    }

    let mut ans = 0;
    for (fstart, (fend, id)) in compressed {
        ans += sum_between(fstart, fend) * id;
    }
    println!("{}", ans);
}
