use std::collections::HashMap;

fn next(last: usize, t: usize, turns: &mut HashMap<usize, Vec<usize>>) -> usize {
    let prev = turns.entry(last).or_default();
    let ret = if prev.len() > 1 {
        prev[prev.len() - 1] - prev[prev.len() - 2]
    } else {
        0
    };
    turns.entry(ret).or_default().push(t);
    ret
}

fn play(input: &[usize], n: usize) -> usize {
    let mut turns: HashMap<_, _> = input
        .iter()
        .enumerate()
        .map(|(n, v)| (*v, vec![n + 1]))
        .collect();
    ((input.len() + 1)..=n).fold(*input.last().unwrap(), |l, t| next(l, t, &mut turns))
}

fn main() {
    let input = [18, 8, 0, 5, 4, 1, 20];
    dbg!(play(&input, 2020));
    dbg!(play(&input, 30000000));
}
