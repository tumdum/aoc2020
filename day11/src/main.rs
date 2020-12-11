use std::io::BufRead;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn ch(&self) -> char {
        match self {
            Seat::Floor => '.',
            Seat::Empty => 'L',
            Seat::Occupied => '#',
        }
    }
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
        map.get(y as usize).and_then(|r| r.get(x as usize)).map(|s| *s)
    }
}

fn new_seat(p: (isize, isize), map: &[Vec<Seat>]) -> Seat {
    let adj : Vec<Seat> = adjacent(p).into_iter().filter_map(|p| get(p, map)).collect();
    let occupied = adj.iter().filter(|s| **s == Seat::Occupied).count();
    match get(p, map) {
        Some(Seat::Empty) if  occupied == 0 => Seat::Occupied,
        Some(Seat::Occupied) if occupied >= 4 => Seat::Empty,
        seat => seat.unwrap(),
    }
}

fn turn(old: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut new = old.clone();
    for y in 0..new.len() {
        for x in 0..new[y].len() {
            new[y][x] = new_seat((x as isize, y as isize), &old)
        }
    }
    new
}

fn find_stable_state(mut current: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    loop {
        let new = turn(current.clone());
        if new == current {
            return new
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
            _ => {},
        }
    }
}

fn new_seat2(p: (isize, isize), map: &[Vec<Seat>]) -> Seat {
    let dirs = vec![(1,0),(-1,0),(0,1),(0,-1),(-1,1),(-1,-1),(1,-1),(1,1)];
    let occupied = dirs.into_iter().filter(|dir| sees_seat_in_dir(p, *dir, map)).count();
    match get(p, map) {
        Some(Seat::Empty) if occupied == 0 => Seat::Occupied,
        Some(Seat::Occupied) if occupied >= 5 => Seat::Empty,
        seat => seat.unwrap(),
    }
}

fn turn2(old: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut new = old.clone();
    for y in 0..new.len() {
        for x in 0..new[y].len() {
            new[y][x] = new_seat2((x as isize, y as isize), &old)
        }
    }
    new
}

fn find_stable_state2(mut current: Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    loop {
        let new = turn2(current.clone());
        if new == current {
            return new
        }
        current = new;
    }
}

fn main() {
    let mut input: Vec<Vec<Seat>> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().chars().map(parse).collect())
        .collect();
    let end = find_stable_state(input.clone());
    dbg!(end.iter().flatten().filter(|s| **s == Seat::Occupied).count());
    let end = find_stable_state2(input.clone());
    dbg!(end.iter().flatten().filter(|s| **s == Seat::Occupied).count());
}
