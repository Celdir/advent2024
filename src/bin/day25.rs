use std::io::{stdin, Read};

fn heights(v: &Vec<&str>) -> [usize; 5] {
    let mut height = [0; 5];
    for s in v {
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            match bytes[i] {
                b'#' => height[i] += 1,
                _ => (),
            };
        }
    }
    height
}

fn main() {
    let mut input = String::new();
    let _ = stdin().lock().read_to_string(&mut input);
    let schematics: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|s| s.split("\n").filter(|s| !s.is_empty()).collect())
        .collect();
    let locks: Vec<_> = schematics
        .iter()
        .filter(|v| v[0] == "#####")
        .map(|v| heights(v))
        .collect();
    let keys: Vec<_> = schematics
        .iter()
        .filter(|v| v[v.len() - 1] == "#####")
        .map(|v| heights(v))
        .collect();
    let mut pairs = 0;
    for lock in &locks {
        for key in &keys {
            let mut fits = true;
            for col in 0..5 {
                if lock[col] + key[col] > 7 {
                    fits = false;
                }
            }
            if fits {
                pairs += 1;
            }
        }
    }
    println!("{}", pairs);
}
