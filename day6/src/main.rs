use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn merge(l: Vec<HashSet<char>>) -> Vec<Vec<HashSet<char>>> {
    l.split(HashSet::is_empty).map(|s| s.to_vec()).collect()
}

fn sum_any(input: &[HashSet<char>]) -> HashSet<char> {
    input
        .iter()
        .cloned()
        .fold1(|a, b| a.into_iter().chain(b.into_iter()).collect())
        .unwrap()
}

fn sum_all(input: &[HashSet<char>]) -> HashSet<char> {
    input
        .iter()
        .cloned()
        .fold1(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
}

fn main() {
    let input = merge(
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap().chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>(),
    );

    dbg!(input.iter().map(|s| sum_any(s).len()).sum::<usize>());
    dbg!(input.iter().map(|s| sum_all(s).len()).sum::<usize>());
}
