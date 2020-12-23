use itertools::Itertools;
type M = std::collections::HashMap<u32, u32>;

fn destination(mut current: u32, min: u32, max: u32, a: u32, b: u32, c: u32) -> u32 {
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

fn take_next(mut from: u32, m: &M, count: usize) -> Vec<u32> {
    let mut ret = vec![];
    while ret.len() < count {
        from = m[&from];
        ret.push(from);
    }
    ret
}

fn solve_fast(input: Vec<u32>, moves: usize) -> (u64, String) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut map: M = input.windows(2).map(|w| (w[0], w[1])).collect();
    map.insert(*input.last().unwrap(), input[0]);
    let mut current = input[0];
    for _ in 1..=moves {
        let pick = take_next(current, &map, 4);
        let dest = destination(current, min, max, pick[0], pick[1], pick[2]);
        let dest_next = map[&dest];
        map.insert(dest, pick[0]);
        map.insert(pick[2], dest_next);
        map.insert(current, pick[3]);
        current = pick[3];
    }
    let tmp = take_next(1, &map, 8);
    let prod = tmp[0] as u64 * tmp[1] as u64;
    (prod, format!("{}", tmp.into_iter().format("")))
}

fn make_big_input(start: &[u32], max: u32) -> Vec<u32> {
    let next = *start.iter().max().unwrap() + 1;
    let mut input: Vec<u32> = start.iter().cloned().collect();
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
