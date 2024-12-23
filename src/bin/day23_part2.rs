use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

use rand::{seq::IteratorRandom, thread_rng};

fn clique<'a>(graph: &'a HashMap<&str, HashSet<&'a str>>, start: &'a str) -> HashSet<&'a str> {
    let mut cur = HashSet::from([start]);
    let mut rng = thread_rng();
    loop {
        let adj: HashSet<&str> = cur
            .iter()
            .map(|s| {
                graph[s]
                    .clone()
                    .into_iter()
                    .filter(|&n| !cur.contains(n))
                    .collect()
            })
            .reduce(|acc, s: HashSet<&str>| acc.intersection(&s).map(|s| s.to_owned()).collect())
            .unwrap();
        if adj.is_empty() {
            break;
        }
        let next = adj.iter().choose(&mut rng).unwrap();
        cur.insert(next);
    }

    cur
}

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let edges = input.iter().map(|l| l.split_once("-").unwrap());
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (u, v) in edges {
        graph.entry(u).or_default().insert(v);
        graph.entry(v).or_default().insert(u);
    }

    let mut rng = thread_rng();
    let mut maxclique: HashSet<&str> = HashSet::new();
    for _ in 0..100 {
        let node = graph.keys().choose(&mut rng).unwrap();
        let c = clique(&graph, node);
        if c.len() > maxclique.len() {
            maxclique = c;
        }
    }
    let mut v: Vec<_> = maxclique.into_iter().collect();
    v.sort();

    println!("{}", v.join(","));
}
