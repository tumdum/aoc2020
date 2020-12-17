use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::Add;

fn offsets_3d() -> Vec<Pos> {
    let mut r = vec![];
    let o = [0, 1, -1];
    for x in &o {
        for y in &o {
            for z in &o {
                if *x != 0 || *y != 0 || *z != 0 {
                    r.push(Pos {
                        x: *x,
                        y: *y,
                        z: *z,
                        w: 0,
                    });
                }
            }
        }
    }
    r
}

fn offsets_4d() -> Vec<Pos> {
    let mut r = vec![];
    let o = [0, 1, -1];
    for x in &o {
        for y in &o {
            for z in &o {
                for w in &o {
                    if *x != 0 || *y != 0 || *z != 0 || *w != 0 {
                        r.push(Pos {
                            x: *x,
                            y: *y,
                            z: *z,
                            w: *w,
                        });
                    }
                }
            }
        }
    }
    r
}

lazy_static::lazy_static! {
    static ref OFFSETS_3D: Vec<Pos> = offsets_3d();
    static ref OFFSETS_4D: Vec<Pos> = offsets_4d();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Pos {
    const fn from_2d(x: isize, y: isize) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    fn neighbours_3d(&self) -> Vec<Self> {
        OFFSETS_3D.iter().map(|o| *self + *o).collect()
    }

    fn neighbours_4d(&self) -> Vec<Self> {
        OFFSETS_4D.iter().map(|o| *self + *o).collect()
    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Active,
    Inactive,
}

impl State {
    fn parse(c: char) -> Self {
        match c {
            '#' => Self::Active,
            '.' => Self::Inactive,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if &Self::Active == self {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

fn parse(row: isize, line: &str) -> Vec<(Pos, State)> {
    line.chars()
        .enumerate()
        .map(|(col, c)| (Pos::from_2d(col as isize, row), State::parse(c)))
        .collect()
}

fn to_check_3d(m: &HashMap<Pos, State>) -> HashSet<Pos> {
    let mut ret: HashSet<Pos> = m.keys().map(|p| *p).collect();
    let n: HashSet<Pos> = ret.iter().flat_map(|p| p.neighbours_3d()).collect();
    ret.extend(n);
    ret
}

fn one_turn(old: HashMap<Pos, State>) -> HashMap<Pos, State> {
    let mut ret = HashMap::new();
    for p in to_check_3d(&old) {
        let active = p
            .neighbours_3d()
            .into_iter()
            .filter_map(|p| old.get(&p))
            .filter(|s| **s == State::Active)
            .count();
        let state = old.get(&p).unwrap_or(&State::Inactive);
        let new = match state {
            State::Active if active == 2 || active == 3 => State::Active,
            State::Active => State::Inactive,
            State::Inactive if active == 3 => State::Active,
            State::Inactive => State::Inactive,
        };
        ret.insert(p, new);
    }
    ret
}

fn min_max(m: &HashMap<Pos, State>) -> (isize, isize, isize, isize) {
    let min_x = m.keys().map(|p| p.x).min().unwrap_or(0);
    let max_x = m.keys().map(|p| p.x).max().unwrap_or(0);
    let min_y = m.keys().map(|p| p.y).min().unwrap_or(0);
    let max_y = m.keys().map(|p| p.y).max().unwrap_or(0);
    (min_x, max_x, min_y, max_y)
}

fn print_slice(m: &HashMap<Pos, State>, z: isize) {
    let m: HashMap<Pos, State> = m
        .iter()
        .filter(|(p, _)| p.z == z)
        .map(|(p, s)| (*p, *s))
        .collect();
    let (min_x, max_x, min_y, max_y) = min_max(&m);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                m.get(&Pos { x, y, z, w: 0 }).unwrap_or(&State::Inactive)
            );
        }
        println!()
    }
}

fn slices(m: &HashMap<Pos, State>) -> std::collections::BTreeSet<isize> {
    m.keys().map(|p| p.z).collect()
}

fn print(m: &HashMap<Pos, State>) {
    for z in slices(&m) {
        println!("z={}", z);
        print_slice(&m, z);
        println!();
    }
}

fn turns(start: HashMap<Pos, State>, n: usize) -> HashMap<Pos, State> {
    let mut ret = start;
    for i in 1..=n {
        ret = one_turn(ret);
        /*
        println!("turn {}", i);
        print(&ret);
        */
    }
    ret
}

fn turns_4d(start: HashMap<Pos, State>, n: usize) -> HashMap<Pos, State> {
    let mut ret = start;
    for i in 1..=n {
        ret = one_turn_4d(ret);
        /*
        println!("turn {}", i);
        print(&ret);
        */
    }
    ret
}

fn one_turn_4d(old: HashMap<Pos, State>) -> HashMap<Pos, State> {
    let mut ret = HashMap::new();
    for p in to_check_4d(&old) {
        let active = p
            .neighbours_4d()
            .into_iter()
            .filter_map(|p| old.get(&p))
            .filter(|s| **s == State::Active)
            .count();
        let state = old.get(&p).unwrap_or(&State::Inactive);
        let new = match state {
            State::Active if active == 2 || active == 3 => State::Active,
            State::Active => State::Inactive,
            State::Inactive if active == 3 => State::Active,
            State::Inactive => State::Inactive,
        };
        ret.insert(p, new);
    }
    ret
}

fn to_check_4d(m: &HashMap<Pos, State>) -> HashSet<Pos> {
    let mut ret: HashSet<Pos> = m.keys().map(|p| *p).collect();
    let n: HashSet<Pos> = ret.iter().flat_map(|p| p.neighbours_4d()).collect();
    ret.extend(n);
    ret
}

fn main() {
    let map = std::io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(row, l)| parse(row as isize, &l.unwrap()))
        .collect::<HashMap<_, _>>();
    dbg!(turns(map.clone(), 6)
        .values()
        .filter(|s| **s == State::Active)
        .count());
    dbg!(turns_4d(map.clone(), 6)
        .values()
        .filter(|s| **s == State::Active)
        .count());
}
