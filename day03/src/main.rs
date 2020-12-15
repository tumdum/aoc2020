use std::io::BufRead;

#[derive(Clone)]
struct Pos {
    x: usize,
    y: usize,
}

fn get(input: &[Vec<char>], pos: &Pos) -> Option<char> {
    input
        .get(pos.y)
        .and_then(|l| l.get(pos.x % l.len()).cloned())
}

fn walk(input: &[Vec<char>], start: &Pos, slope: &Pos) -> Vec<char> {
    let mut ret = vec![];
    let mut c = start.clone();
    while let Some(v) = get(&input, &c) {
        ret.push(v);
        c.x += slope.x;
        c.y += slope.y;
    }
    ret
}

fn solve_a(input: &[Vec<char>], start: &Pos, slope: &Pos) -> usize {
    walk(&input, start, slope)
        .into_iter()
        .filter(|c| *c == '#')
        .count()
}

fn solve_b(input: &[Vec<char>], start: &Pos) -> usize {
    let slopes = &[
        Pos { x: 1, y: 1 },
        Pos { x: 3, y: 1 },
        Pos { x: 5, y: 1 },
        Pos { x: 7, y: 1 },
        Pos { x: 1, y: 2 },
    ];
    slopes.iter().map(|s| solve_a(input, start, &s)).product()
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect();
    dbg!(solve_a(&input, &Pos { x: 0, y: 0 }, &Pos { x: 3, y: 1 }));
    dbg!(solve_b(&input, &Pos { x: 0, y: 0 }));
}
