use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn is_sorted(v: &Vec<usize>) -> bool {
    for i in 1..v.len() {
        if v[i] < v[i - 1] {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut input = stdin().lock().lines().map(|l| l.unwrap()).into_iter();
    let rules: Vec<(String, String)> = input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (u, v) = l.split_once("|").unwrap();
            (u.into(), v.into())
        })
        .collect();
    let updates: Vec<Vec<String>> = input
        .by_ref()
        .map(|l| l.split(",").map(|s| s.into()).collect())
        .collect();

    let mut ans = 0;

    for update in &updates {
        let updateset: HashSet<_> = HashSet::from_iter(update.iter().cloned());
        let mut in_degree: HashMap<_, usize> = HashMap::new();
        let mut graph: HashMap<_, Vec<_>> = HashMap::new();
        for (u, v) in rules
            .iter()
            .filter(|(u, v)| updateset.contains(u) && updateset.contains(v))
        {
            *in_degree.entry(v).or_default() += 1;
            graph.entry(u).or_default().push(v);
            graph.entry(v).or_default();
        }

        // note: I did topological groups like this instead of just a list because in theory there are multiple valid
        // topological orderings for a given rule set. In practice it seems that the given inputs
        // have a unique topological ordering per update.
        let mut topological_groups: Vec<Vec<String>> = Vec::new();
        let mut queue: Vec<String> = Vec::new();
        for node in graph.keys() {
            let &degree = in_degree.get(node).unwrap_or(&0);
            if degree == 0 {
                queue.push(node.to_string());
            }
        }
        while !queue.is_empty() {
            topological_groups.push(queue.clone());
            let mut next_queue = Vec::new();
            for node in &queue {
                for adj in &graph[&node] {
                    let deg = in_degree.entry(adj).or_default();
                    *deg -= 1;
                    if *deg == 0 {
                        next_queue.push(adj.to_string());
                    }
                }
            }
            queue = next_queue;
        }

        let mut groupidx = HashMap::new();
        for i in 0..topological_groups.len() {
            for node in &topological_groups[i] {
                groupidx.insert(node, i);
            }
        }

        let mut orderidx = Vec::new();
        for node in update {
            if groupidx.contains_key(&node) {
                orderidx.push(groupidx[node]);
            }
        }
        if is_sorted(&orderidx) {
            let middle: usize = update[update.len() / 2].parse().unwrap();
            ans += middle;
        }
    }

    println!("{}", ans);
}
