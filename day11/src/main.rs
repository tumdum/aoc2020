use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn parse(c: char) -> Seat {
    match c {
        '.' => Seat::Floor,
        'L' => Seat::Empty,
        '#' => Seat::Occupied,
        _ => unreachable!(),
    }
}

fn adjacent((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ]
}

fn get((x, y): (isize, isize), map: &[Vec<Seat>]) -> Option<Seat> {
    if x < 0 || y < 0 {
        None
    } else {
        map.get(y as usize)
            .and_then(|r| r.get(x as usize))
            .map(|s| *s)
    }
}

fn new_seat(p: (isize, isize), map: &[Vec<Seat>]) -> Seat {
    let occupied = adjacent(p)
        .into_iter()
        .filter(|p| get(*p, map) == Some(Seat::Occupied))
        .count();
    match get(p, map) {
        Some(Seat::Empty) if occupied == 0 => Seat::Occupied,
        Some(Seat::Occupied) if occupied >= 4 => Seat::Empty,
        seat => seat.unwrap(),
    }
}

fn turn(old: &[Vec<Seat>], ns: impl Fn((isize, isize), &[Vec<Seat>]) -> Seat) -> Vec<Vec<Seat>> {
    let mut new = old.to_vec();
    for y in 0..new.len() {
        for x in 0..new[y].len() {
            new[y][x] = ns((x as isize, y as isize), &old)
        }
    }
    new
}

fn find_stable_state(
    mut current: Vec<Vec<Seat>>,
    ns: impl Fn((isize, isize), &[Vec<Seat>]) -> Seat,
) -> Vec<Vec<Seat>> {
    loop {
        let new = turn(&current, &ns);
        if new == current {
            return new;
        }
        current = new;
    }
}

fn sees_seat_in_dir(mut current: (isize, isize), dir: (isize, isize), m: &[Vec<Seat>]) -> bool {
    loop {
        current = (current.0 + dir.0, current.1 + dir.1);
        match get(current, m) {
            None | Some(Seat::Empty) => return false,
            Some(Seat::Occupied) => return true,
            _ => {}
        }
    }
}

fn new_seat_part2(p: (isize, isize), map: &[Vec<Seat>]) -> Seat {
    let dirs = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, -1),
        (1, 1),
    ];
    let occupied = dirs
        .into_iter()
        .filter(|dir| sees_seat_in_dir(p, *dir, map))
        .count();
    match get(p, map) {
        Some(Seat::Empty) if occupied == 0 => Seat::Occupied,
        Some(Seat::Occupied) if occupied >= 5 => Seat::Empty,
        seat => seat.unwrap(),
    }
}

fn main() {
    let input: Vec<Vec<Seat>> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(parse).collect())
        .collect();
    let end = find_stable_state(input.clone(), new_seat);
    dbg!(end
        .iter()
        .flatten()
        .filter(|s| **s == Seat::Occupied)
        .count());
    let end = find_stable_state(input.clone(), new_seat_part2);
    dbg!(end
        .iter()
        .flatten()
        .filter(|s| **s == Seat::Occupied)
        .count());
}
