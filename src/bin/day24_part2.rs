use std::io::{stdin, Read};

#[derive(PartialEq, Eq, Copy, Clone)]
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
    let (_, gates_in) = input.split_once("\n\n").unwrap();
    let gates: Vec<_> = gates_in
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(lrop, out)| {
            let sp: Vec<_> = lrop.split(" ").collect();
            let (left, right) = match sp[0] {
                s if s.starts_with("y") && sp[2].starts_with("x") => (sp[2], sp[0]),
                _ => (sp[0], sp[2]),
            };
            Gate {
                left,
                op: Op::parse(sp[1]),
                right,
                out,
            }
        })
        .collect();
    let mut bad = vec![];
    for gate in &gates {
        let out = gate.out;
        if out.starts_with("z") && !out.ends_with("45") {
            if gate.op != Op::XOR {
                bad.push(out);
            }
        } else if !(gate.left.starts_with("x") || gate.right.starts_with("y")) {
            if gate.op == Op::XOR {
                bad.push(out);
            }
        } else if gate.left.starts_with("x") && gate.right.starts_with("y") {
            if gate.left.ends_with("00") || gate.right.ends_with("00") {
                continue;
            }
            let mut downstream = vec![];
            for other in &gates {
                if other.left == out || other.right == out {
                    downstream.push(other.op);
                }
            }

            if gate.op == Op::XOR && !downstream.contains(&Op::XOR)
                || gate.op == Op::AND && !downstream.contains(&Op::OR)
            {
                bad.push(out);
            }
        }
    }
    bad.sort();
    println!("{}", bad.join(","));
}
