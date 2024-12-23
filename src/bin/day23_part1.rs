use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let edges = input.iter().map(|l| l.split_once("-").unwrap());
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (u, v) in edges {
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }
    let ts = graph.keys().filter(|k| k.starts_with("t"));
    let mut cliques = HashSet::new();
    for t in ts {
        for k1 in graph.keys() {
            if k1 == t || !graph[t].contains(k1) {
                continue;
            }
            for k2 in graph.keys() {
                if k2 == t || k2 == k1 || !graph[t].contains(k2) || !graph[k1].contains(k2) {
                    continue;
                }
                let mut c = [k1.to_string(), k2.to_string(), t.to_string()];
                c.sort();
                cliques.insert(c.join(","));
            }
        }
    }

    println!("{}", cliques.len());
}
