use itertools::Itertools;
type M = Vec<usize>;

fn destination(mut current: usize, min: usize, max: usize, a: usize, b: usize, c: usize) -> usize {
    loop {
        current -= 1;
        if current < min {
            current = max;
        }
        if current != a && current != b && current != c {
            return current;
        }
    }
}

fn take_next(mut from: usize, m: &M, count: usize) -> Vec<usize> {
    let mut ret = vec![];
    while ret.len() < count {
        from = m[from];
        ret.push(from);
    }
    ret
}

fn solve_fast(input: Vec<usize>, moves: usize) -> (u64, String) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut next: Vec<usize> = vec![0; input.len() + 1];
    for v in input.windows(2) {
        next[v[0]] = v[1];
    }
    next[*input.last().unwrap()] = input[0];
    let mut current = input[0];
    for _ in 1..=moves {
        let pick = take_next(current, &next, 4);
        let dest = destination(current, min, max, pick[0], pick[1], pick[2]);
        let dest_next = next[dest];
        next[dest] = pick[0];
        next[pick[2]] = dest_next;
        next[current] = pick[3];
        current = pick[3];
    }
    let tmp = take_next(1, &next, 8);
    let prod = tmp[0] as u64 * tmp[1] as u64;
    (prod, format!("{}", tmp.into_iter().format("")))
}

fn make_big_input(start: &[usize], max: usize) -> Vec<usize> {
    let next = *start.iter().max().unwrap() + 1;
    let mut input: Vec<usize> = start.iter().cloned().collect();
    input.extend(next..=max);
    input
}

fn main() {
    let total_size = 1_000_000;
    let rounds = 10_000_000;
    let input = vec![2, 1, 9, 7, 4, 8, 3, 6, 5];
    dbg!(&solve_fast(input.clone(), 100).1);
    dbg!(solve_fast(make_big_input(&input, total_size), rounds).0);
}
