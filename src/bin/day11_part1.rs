use std::io::{stdin, BufRead};

fn blink(x: usize, it: usize) -> usize {
    if it >= 25 {
        return 1;
    }
    match x {
        0 => blink(1, it + 1),
        x if x.to_string().len() % 2 == 0 => {
            let xstr = x.to_string();
            let len = xstr.len();
            return blink(xstr[0..len / 2].parse().unwrap(), it + 1)
                + blink(xstr[len / 2..len].parse().unwrap(), it + 1);
        }
        _ => blink(x * 2024, it + 1),
    }
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
    for x in input {
        ans += blink(x, 0);
    }
    println!("{}", ans);
}
