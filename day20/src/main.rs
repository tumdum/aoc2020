use std::collections::{HashMap, HashSet};
use std::io::BufRead;

type Tile = Vec<Vec<bool>>;
type Map = HashMap<usize, HashSet<Tile>>;

fn parse(lines: &[String]) -> (usize, Tile) {
    let name = lines[0]
        .split(' ')
        .nth(1)
        .map(|s| s[0..s.len() - 1].parse().unwrap())
        .unwrap();
    let tile: Tile = lines[1..]
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();
    assert_eq!(
        1,
        tile.iter().map(|l| l.len()).collect::<HashSet<_>>().len()
    );
    (name, tile)
}

fn horizontal_flip(t: &Tile) -> Tile {
    t.iter()
        .map(|l| l.iter().rev().cloned().collect())
        .collect()
}

fn vertical_flip(t: &Tile) -> Tile {
    t.iter().rev().cloned().collect()
}

// All possible transformation of tile (rotate, both flips)
fn possible(t: &Tile) -> HashSet<Tile> {
    let mut s = HashSet::new();
    s.extend(all_possible_rotations(t));
    s.extend(all_possible_rotations(&horizontal_flip(t)));
    s.extend(all_possible_rotations(&vertical_flip(t)));
    s
}

fn rotate_right(t: &Tile) -> Tile {
    let mut copy = t.clone();
    let l = t.len();
    for y in 0..l {
        assert_eq!(l, t[y].len());
        for x in 0..l {
            copy[x][l - 1 - y] = t[y][x];
        }
    }
    copy
}

fn all_possible_rotations(t: &Tile) -> HashSet<Tile> {
    let mut s = HashSet::new();

    s.insert(t.clone());

    let t = rotate_right(&t);
    s.insert(t.clone());

    let t = rotate_right(&t);
    s.insert(t.clone());

    let t = rotate_right(&t);
    s.insert(t);
    s
}

#[allow(dead_code)]
fn print(t: &Tile) {
    let l = t.len();
    for y in 0..l {
        for x in 0..t[y].len() {
            if t[y][x] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

// Get values (one row or column) on 'side' of tile
fn get_side(t: &Tile, side: Side) -> Vec<bool> {
    match side {
        Side::Up => t[0].clone(),
        Side::Down => t.last().unwrap().clone(),
        Side::Left => t.iter().map(|l| l[0]).collect(),
        Side::Right => t.iter().map(|l| *l.last().unwrap()).collect(),
    }
}

// Check if base and other have same values on side and opposite side respecitvely
fn match_on(base: &Tile, other: &Tile, side: Side) -> bool {
    get_side(base, side) == get_side(other, side.opposite())
}

// Find a tile in all that is not a base and matches on side with base
fn find_match(base: &(usize, Tile), side: Side, all: &Map) -> Option<(usize, Tile)> {
    let v: Vec<(usize, Tile)> = all
        .iter()
        .filter(|(id, _)| **id != base.0)
        .flat_map(|(id, possible)| {
            possible
                .iter()
                .filter(|t| match_on(&base.1, t, side))
                .map(move |t| (*id, t.clone()))
        })
        .collect();
    if v.len() == 1 {
        v.into_iter().next()
    } else {
        None
    }
}

// Add one tile on both left and right side that matches with each end
fn expand_row_once(r: &[(usize, Tile)], all: &Map) -> Vec<(usize, Tile)> {
    let mut r = r.to_vec();
    if let Some(lt) = find_match(&r[0], Side::Left, all) {
        r.insert(0, lt);
    }
    if let Some(rt) = find_match(&r.last().unwrap(), Side::Right, all) {
        r.push(rt)
    }
    r
}

// Expend row in both left and right until there is no way to do that
fn expand_row(r: &[(usize, Tile)], all: &Map) -> Vec<(usize, Tile)> {
    let mut old = r.to_vec();
    loop {
        let new = expand_row_once(&old, all);
        if new == old {
            return old;
        }
        old = new;
    }
}

// Find a new row that matches 'row' on side (up or down)
fn row_to_side(row: &[(usize, Tile)], side: Side, all: &Map) -> Option<Vec<(usize, Tile)>> {
    let mut other_row = vec![];
    for t in row {
        if let Some(t) = find_match(t, side, all) {
            other_row.push(t);
        } else {
            return None;
        }
    }
    Some(other_row)
}

// Expand rows in up/down direction by at most one new row in each direction
fn expand_once(rows: &[Vec<(usize, Tile)>], all: &Map) -> Vec<Vec<(usize, Tile)>> {
    let mut rows = rows.to_vec();
    if let Some(row) = row_to_side(&rows[0], Side::Up, all) {
        rows.insert(0, row);
    }
    if let Some(row) = row_to_side(&rows.last().unwrap(), Side::Down, all) {
        rows.push(row);
    }
    rows
}

// Expand rows in up/down until there is no way to do that
fn expand(rows: &[Vec<(usize, Tile)>], all: &Map) -> Vec<Vec<(usize, Tile)>> {
    let mut old = rows.to_vec();
    loop {
        let new = expand_once(&old, all);
        if new == old {
            return old;
        }
        old = new;
    }
}

fn get_ids(solved: &[Vec<(usize, Tile)>]) -> Vec<Vec<usize>> {
    solved
        .iter()
        .map(|row| row.iter().map(|v| v.0).collect::<Vec<_>>())
        .collect()
}

fn merge_row(row: &[(usize, Tile)]) -> Vec<Vec<bool>> {
    (1..row[0].1.len() - 1)
        .map(|y| {
            row.iter()
                .flat_map(|(_, tile)| tile[y][1..tile[y].len() - 1].to_vec())
                .collect()
        })
        .collect()
}

fn merge(solved: &[Vec<(usize, Tile)>]) -> Tile {
    solved.iter().flat_map(|row| merge_row(row)).collect()
}

fn monster() -> Tile {
    let input = vec![
        "t 1:".to_owned(),
        "                  # ".to_owned(),
        "#    ##    ##    ###".to_owned(),
        " #  #  #  #  #  #   ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
        "                    ".to_owned(),
    ];
    parse(&input).1
}

fn is_on((x, y): (usize, usize), map: &Tile) -> bool {
    let row = if let Some(row) = map.get(y) {
        row
    } else {
        return false;
    };
    *row.get(x).unwrap_or(&false)
}

fn matches(what: &Tile, sea: &Tile, start: (isize, isize)) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();
    for y in 0..what.len() {
        for x in 0..what[y].len() {
            let pos = (x, y);
            let monster = is_on(pos, what);
            if monster {
                let sea_pos = (x as isize + start.0, y as isize + start.1);
                let sea_pos = if sea_pos.0 < 0 || sea_pos.1 < 0 {
                    return HashSet::new();
                } else {
                    (sea_pos.0 as usize, sea_pos.1 as usize)
                };
                let sea = is_on(sea_pos, sea);
                if sea {
                    ret.insert(sea_pos);
                } else {
                    return HashSet::new();
                }
            }
        }
    }
    ret
}

fn main() {
    let input: HashMap<_, _> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .split(String::is_empty)
        .map(|s| parse(s))
        .collect();
    let t = input.values().next().unwrap();

    // tests
    assert_eq!(
        t,
        &rotate_right(&rotate_right(&rotate_right(&rotate_right(t))))
    );
    assert_eq!(t, &horizontal_flip(&horizontal_flip(t)));
    assert_eq!(t, &vertical_flip(&vertical_flip(t)));
    assert_eq!(4, all_possible_rotations(t).len());

    assert!(match_on(t, &vertical_flip(t), Side::Up));
    assert!(match_on(t, &vertical_flip(t), Side::Down));

    assert!(match_on(t, &horizontal_flip(t), Side::Left));
    assert!(match_on(t, &horizontal_flip(t), Side::Right));

    // part 1
    let all: Map = input
        .iter()
        .map(|(id, base)| (*id, possible(base)))
        .collect();
    let t = input.iter().next().map(|(id, t)| (*id, t.clone())).unwrap();
    let rows = vec![expand_row(&vec![t], &all)];
    let solved = expand(&rows, &all);
    let ids = get_ids(&solved);
    let a = ids[0][0];
    let b = ids[0].last().unwrap();
    let c = ids.last().unwrap()[0];
    let d = ids.last().unwrap().last().unwrap();
    dbg!(a * b * c * d);

    // part 2
    let stiched = merge(&solved);
    let all_on: usize = stiched
        .iter()
        .map(|row| row.iter().filter(|b| **b).count())
        .sum();
    let mut all_monster_on = HashSet::new();
    let offset = 20;
    for m in possible(&monster()) {
        for y in -offset..(stiched.len() as isize + offset) {
            for x in -offset..(stiched[0].len() as isize + offset) {
                let result = matches(&m, &stiched, (x, y));
                if result.len() == 15 {
                    all_monster_on.extend(result);
                }
            }
        }
    }
    dbg!(all_on - all_monster_on.len());
}
