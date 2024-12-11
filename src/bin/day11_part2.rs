use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn blink(memo: &mut HashMap<(usize, usize), usize>, x: usize, it: usize) -> usize {
    if let Some(&ans) = memo.get(&(x, it)) {
        return ans;
    }
    if it >= 75 {
        return 1;
    }
    let ans = match x {
        0 => blink(memo, 1, it + 1),
        x if x.to_string().len() % 2 == 0 => {
            let xstr = x.to_string();
            let len = xstr.len();
            blink(memo, xstr[0..len / 2].parse().unwrap(), it + 1)
                + blink(memo, xstr[len / 2..len].parse().unwrap(), it + 1)
        }
        _ => blink(memo, x * 2024, it + 1),
    };
    memo.insert((x, it), ans);
    ans
}

fn main() {
    let input: Vec<usize> = stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut ans = 0;
    let mut memo = HashMap::new();
    for x in input {
        ans += blink(&mut memo, x, 0);
    }
    println!("{}", ans);
}
