use itertools::Itertools;
use std::io::BufRead;
type Set = std::collections::HashSet<char>;

fn sum(input: &[Set], f: impl FnMut(Set, Set) -> Set) -> usize {
    input.iter().cloned().fold1(f).map_or(0, |s| s.len())
}

fn main() {
    let input: Vec<Vec<Set>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Set>())
        .collect::<Vec<_>>()
        .split(Set::is_empty)
        .map(|s| s.to_vec())
        .collect();

    dbg!(input
        .iter()
        .map(|s| sum(s, |a, b| a.into_iter().chain(b.into_iter()).collect()))
        .sum::<usize>());
    dbg!(input
        .iter()
        .map(|s| sum(s, |a, b| a.intersection(&b).cloned().collect()))
        .sum::<usize>());
}
