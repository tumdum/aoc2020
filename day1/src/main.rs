use std::collections::HashMap;
use std::io::BufRead;

fn prepare(input: &[usize]) -> HashMap<usize, (usize, usize)> {
    let mut m = HashMap::new();
    for a in input {
        for b in input {
            m.insert(a + b, (*a, *b));
        }
    }
    m
}

fn part_1(input: &[usize]) -> Option<usize> {
    prepare(input).get(&2020).map(|v| v.0 * v.1)
}

fn part_2(input: &[usize]) -> Option<usize> {
    let m = prepare(input);
    for v in input {
        if let Some((a, b)) = m.get(&(2020 - v)) {
            return Some(a * b * v);
        }
    }
    None
}
fn main() {
    let input: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("{:?}", part_1(&input));
    println!("{:?}", part_2(&input));
}
