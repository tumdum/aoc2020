use itertools::{iterate, Itertools};
use std::collections::HashSet;
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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    col: isize,
    row: isize,
}

impl Pos {
    fn all_neighbours(self) -> impl Iterator<Item = Pos> {
        [E, W, SE, SW, NE, NW]
            .iter()
            .map(move |d| self.neighbour(*d))
    }
    fn neighbour(self, dir: Dir) -> Self {
        match dir {
            W => Pos {
                col: self.col - 2,
                row: self.row,
            },
            E => Pos {
                col: self.col + 2,
                row: self.row,
            },
            SE => Pos {
                col: self.col + 1,
                row: self.row + 1,
            },
            NW => Pos {
                col: self.col - 1,
                row: self.row - 1,
            },
            SW => Pos {
                col: self.col - 1,
                row: self.row + 1,
            },
            NE => Pos {
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
            ret.push(E);
            s = &s[1..];
        } else if s.starts_with("se") {
            ret.push(SE);
            s = &s[2..];
        } else if s.starts_with("sw") {
            ret.push(SW);
            s = &s[2..];
        } else if s.starts_with("w") {
            ret.push(W);
            s = &s[1..];
        } else if s.starts_with("nw") {
            ret.push(NW);
            s = &s[2..];
        } else if s.starts_with("ne") {
            ret.push(NE);
            s = &s[2..];
        }
    }
    ret
}

fn should_be_black(p: Pos, prev: &HashSet<Pos>) -> bool {
    let black_neighbours = p.all_neighbours().filter(|p| prev.contains(p)).count();
    black_neighbours == 2 || (black_neighbours == 1 && prev.contains(&p))
}

fn round(black: &HashSet<Pos>) -> HashSet<Pos> {
    black
        .iter()
        .cloned()
        .chain(black.iter().flat_map(|p| p.all_neighbours()))
        .filter(|p| should_be_black(*p, black))
        .collect()
}

fn main() {
    let h: HashSet<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| Pos { col: 0, row: 0 }.move_by(&parse(&l.unwrap())))
        .sorted()
        .group_by(|p| *p)
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().count()))
        .filter(|(_, v)| *v % 2 == 1)
        .map(|(p, _)| p)
        .collect();
    dbg!(h.len());
    dbg!(iterate(h, round).nth(100).map(|h| h.len()));
}

#[test]
fn move_by() {
    for col in -10..10 {
        for row in -10..10 {
            let s = Pos { col, row };
            assert_eq!(s, s.move_by(&parse("nwwswee")));
        }
    }
}
