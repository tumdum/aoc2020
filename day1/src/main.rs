use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

fn prepare(input: &[usize]) -> HashMap<usize, (usize, usize)> {
    input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| (a + b, (*a, *b)))
        .collect()
}

fn part_1(input: &[usize]) -> Option<usize> {
    prepare(input).get(&2020).map(|v| v.0 * v.1)
}

fn part_2(input: &[usize]) -> Option<usize> {
    let m = prepare(input);
    input
        .iter()
        .find_map(|v| m.get(&(2020 - *v)).map(|(a, b)| a * b * *v))
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}
