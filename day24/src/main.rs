use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}
use Dir::*;

// double-width variant from https://www.redblobgames.com/grids/hexagons/#coordinates
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    col: isize,
    row: isize,
}

impl Pos {
    fn all_neighbours(self) -> Vec<Self> {
        [E, W, SE, SW, NE, NW]
            .iter()
            .map(|d| self.neighbour(*d))
            .collect()
    }
    fn neighbour(self, dir: Dir) -> Self {
        match dir {
            Dir::W => Pos {
                col: self.col - 2,
                row: self.row,
            },
            Dir::E => Pos {
                col: self.col + 2,
                row: self.row,
            },
            Dir::SE => Pos {
                col: self.col + 1,
                row: self.row + 1,
            },
            Dir::NW => Pos {
                col: self.col - 1,
                row: self.row - 1,
            },
            Dir::SW => Pos {
                col: self.col - 1,
                row: self.row + 1,
            },
            Dir::NE => Pos {
                col: self.col + 1,
                row: self.row - 1,
            },
        }
    }

    fn move_by(self, dir: &[Dir]) -> Self {
        dir.iter().fold(self, |p, d| p.neighbour(*d))
    }
}

fn parse(mut s: &str) -> Vec<Dir> {
    let mut ret = vec![];
    while !s.is_empty() {
        if s.starts_with("e") {
            ret.push(Dir::E);
            s = &s[1..];
        } else if s.starts_with("se") {
            ret.push(Dir::SE);
            s = &s[2..];
        } else if s.starts_with("sw") {
            ret.push(Dir::SW);
            s = &s[2..];
        } else if s.starts_with("w") {
            ret.push(Dir::W);
            s = &s[1..];
        } else if s.starts_with("nw") {
            ret.push(Dir::NW);
            s = &s[2..];
        } else if s.starts_with("ne") {
            ret.push(Dir::NE);
            s = &s[2..];
        }
    }
    ret
}

fn should_be_black(p: Pos, prev: &HashSet<Pos>) -> bool {
    let neighbours = p.all_neighbours();
    let black_neighbours = neighbours.iter().filter(|p| prev.contains(p)).count();
    let is_black = prev.contains(&p);
    (is_black && (black_neighbours == 1 || black_neighbours == 2)) || black_neighbours == 2
}

fn round(black: &HashSet<Pos>) -> HashSet<Pos> {
    let mut to_consider = black.clone();
    to_consider.extend(black.iter().flat_map(|p| p.all_neighbours()));
    to_consider
        .into_iter()
        .filter(|p| should_be_black(*p, black))
        .collect()
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse(&l.unwrap()))
        .collect();

    for col in -10..10 {
        for row in -10..10 {
            let s = Pos { col, row };
            assert_eq!(s, s.move_by(&parse("nwwswee")));
        }
    }
    let start = Pos { col: 0, row: 0 };
    let mut v: Vec<_> = input.iter().map(|d| start.move_by(&d)).collect();
    let mut h = HashMap::new();
    for (i, d) in v.iter().enumerate() {
        *h.entry(d).or_insert(0) += 1;
    }
    let mut h: HashSet<Pos> = h
        .iter()
        .filter(|(_, v)| **v % 2 == 1)
        .map(|(p, _)| **p)
        .collect();
    dbg!(h.len());
    for r in 1..=100 {
        h = round(&h);
    }
    println!("{}", h.len());
}
