use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead, Read},
};

enum Op {
    AND,
    OR,
    XOR,
}

impl Op {
    fn parse(s: &str) -> Op {
        match s {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => panic!("invalid op"),
        }
    }

    fn call(&self, left: u64, right: u64) -> u64 {
        match &self {
            Op::AND => left & right,
            Op::OR => left | right,
            Op::XOR => left ^ right,
        }
    }
}

struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    out: &'a str,
    op: Op,
}

fn main() {
    let mut input = String::new();
    let _ = stdin().lock().read_to_string(&mut input);
    let (values_in, gates_in) = input.split_once("\n\n").unwrap();
    let mut values: HashMap<&str, u64> = values_in
        .split("\n")
        .map(|s| s.split_once(": ").unwrap())
        .map(|(name, val)| (name, val.parse().unwrap()))
        .collect();
    let gates: Vec<_> = gates_in
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(lrop, out)| {
            let sp: Vec<_> = lrop.split(" ").collect();
            Gate {
                left: sp[0],
                op: Op::parse(sp[1]),
                right: sp[2],
                out,
            }
        })
        .collect();
    let mut queue = vec![];
    let mut adj: HashMap<&str, Vec<(&str, usize)>> = HashMap::new();
    for i in 0..gates.len() {
        if values.contains_key(gates[i].left) && values.contains_key(gates[i].right) {
            queue.push(i);
        }
        adj.entry(gates[i].left)
            .or_default()
            .push((gates[i].right, i));
        adj.entry(gates[i].right)
            .or_default()
            .push((gates[i].left, i));
    }

    while !queue.is_empty() {
        let gate = &gates[queue.pop().unwrap()];
        let result = gate.op.call(values[gate.left], values[gate.right]);
        values.insert(gate.out, result);

        if let Some(neighbors) = adj.get(gate.out) {
            for &(other, i) in neighbors {
                if values.contains_key(other) {
                    queue.push(i);
                }
            }
        }
    }

    let mut ans = 0;
    for (name, val) in &values {
        if name.starts_with("z") {
            let pos: usize = name.strip_prefix("z").unwrap().parse().unwrap();
            ans |= val << pos;
        }
    }
    println!("{}", ans);
}

