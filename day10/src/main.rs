use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn diffs(input: &[usize]) -> HashMap<usize, usize> {
    let mut m = HashMap::new();
    for d in input.windows(2).map(|v| v[1] - v[0]) {
        *m.entry(d).or_insert(0) += 1
    }
    m
}

fn can_go_to(from: usize, to: usize) -> bool {
    !(to <= from || to > (from + 3))
}

fn solve_b(input: &[usize]) -> usize {
    let forward: HashMap<usize, HashSet<usize>> = input
        .iter()
        .map(|v| {
            (
                *v,
                input
                    .iter()
                    .filter(|o| can_go_to(*v, **o))
                    .map(|v| *v)
                    .collect(),
            )
        })
        .collect();
    let mut backward = HashMap::new();
    for (from, to) in forward {
        for t in to {
            backward.entry(t).or_insert(HashSet::new()).insert(from);
        }
    }
    let mut paths_to = HashMap::new();
    paths_to.insert(0, 1);
    for i in &input[1..] {
        paths_to.insert(*i, backward[i].iter().map(|v| paths_to[v]).sum::<usize>());
    }
    paths_to[&input[input.len() - 1]]
}

fn main() {
    let mut input: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    let max = input.iter().max().unwrap() + 3;
    input.push(max);
    input.push(0);
    input.sort();
    let diffs = diffs(&input);
    dbg!(diffs[&1] * diffs[&3]);
    dbg!(solve_b(&input));
}
