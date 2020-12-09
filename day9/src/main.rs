use itertools::Itertools;
use std::io::BufRead;

fn is_sum_of(nums: &[usize], sum: usize) -> bool {
    nums.iter()
        .combinations(2)
        .map(|p| p.into_iter().sum::<usize>())
        .find(|s| *s == sum)
        .is_some()
}

fn check(input: &[usize], len: usize) -> Option<usize> {
    input
        .windows(len + 1)
        .find(|w| !is_sum_of(&w[..len], w[len]))
        .map(|w| w[len])
}

fn find_sum(input: &[usize], start: usize, sum: usize) -> Option<Vec<usize>> {
    if input[start] == sum {
        return None;
    }
    let mut s = 0;
    let mut current = start;
    let mut v = vec![];
    loop {
        if current >= input.len() {
            return None;
        }
        s += input[current];
        v.push(input[current]);
        if s == sum {
            return Some(v);
        }
        if s > sum {
            return None;
        }
        current += 1;
    }
}

fn main() {
    let input: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    let a = dbg!(check(&input, 25)).unwrap();
    let b = (0..input.len())
        .filter_map(|start| find_sum(&input, start, a))
        .collect::<Vec<_>>()[0]
        .clone();
    dbg!(b.iter().max().unwrap() + b.iter().min().unwrap());
}
