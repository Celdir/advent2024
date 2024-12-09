use std::{
    cmp::min,
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
    for (mut start, end) in free_intervals {
        while let Some((fstart, (fend, id))) = file_intervals.pop_last() {
            if fstart < start || start >= end {
                file_intervals.insert(fstart, (fend, id));
                break;
            }
            let moved = min(end - start, fend - fstart);
            file_intervals.insert(start, (start + moved, id));
            if moved < fend - fstart {
                file_intervals.insert(fstart, (fstart + (fend - fstart) - moved, id));
            }
            start += moved;
        }
    }

    let mut ans = 0;
    for (fstart, (fend, id)) in file_intervals {
        ans += sum_between(fstart, fend) * id;
    }
    println!("{}", ans);
}
