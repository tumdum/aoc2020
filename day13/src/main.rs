use std::collections::BTreeMap;
use std::io::BufRead;

fn next_time(target: u64, div: u64) -> u64 {
    if target % div == 0 {
        target
    } else {
        div * ((target / div) + 1)
    }
}

fn is_good(t: u64, m: &[(u64, u64)]) -> bool {
    for (id, diff) in m {
        let t = t + diff;
        if t % id != 0 {
            return false;
        }
    }
    true
}

fn solve_b_slow(input: &[(u64, u64)], start: usize, step: u64) -> (u64, usize) {
    let offset = offset(&input.iter().map(|v| v.0).collect::<Vec<_>>()) as usize;
    (
        (start..)
            .step_by(step as usize)
            .find(|t| is_good(*t as u64, input))
            .unwrap() as u64,
        offset,
    )
}

fn offset(input: &[u64]) -> u64 {
    input.iter().fold(input[0], |a, b| num::integer::lcm(a, *b))
}

fn solve_b(input: &[Option<u64>]) -> u64 {
    let mut start = 0;
    let mut step = 1;
    let busses: Vec<(u64, u64)> = input
        .into_iter()
        .enumerate()
        .filter_map(|(n, v)| v.map(|v| (v, n as u64)))
        .collect::<Vec<_>>();
    for l in 1..=busses.len() {
        let (new_start, new_step) = solve_b_slow(&busses[..l], start, step);
        start = new_start as usize;
        step = new_step as u64;
    }
    start as u64
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap())
        .collect();
    let us: u64 = input[0].parse().unwrap();
    let busses = input[1]
        .split(',')
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let next: BTreeMap<_, _> = busses
        .iter()
        .filter_map(|v| v.clone())
        .map(|v| (next_time(us, v), v))
        .collect();
    dbg!(next.iter().next().map(|(t, id)| (t - us) * id));
    dbg!(solve_b(&busses));
}

#[test]
fn solve_b_test() {
    assert_eq!(
        false,
        is_good(50400, &vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)])
    );
    assert_eq!(
        1068781,
        solve_b_slow(&vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)], 1)
    );
    assert_eq!(3417, solve_b_slow(&vec![(17, 0), (13, 2), (19, 3)], 1));
    assert_eq!(
        754018,
        solve_b_slow(&vec![(67, 0), (7, 1), (59, 2), (61, 3)], 1)
    );
    assert_eq!(
        1202161486,
        solve_b_slow(&vec![(1789, 0), (37, 1), (47, 2), (1889, 3)], 1)
    );
}
