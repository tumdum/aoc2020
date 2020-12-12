use std::io::BufRead;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn turn(&self, turn: &Turn, val: isize) -> Self {
        fn turn_once(d: Dir, t: &Turn) -> Dir {
            use Dir::*;
            use Turn::*;
            match (d, t) {
                (N, L) => W,
                (N, R) => E,
                (S, L) => E,
                (S, R) => W,
                (E, L) => N,
                (E, R) => S,
                (W, L) => S,
                (W, R) => N,
            }
        }
        let mut ret = self.clone();
        for _ in 0..(val / 90) {
            ret = turn_once(ret, turn);
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Turn {
    L,
    R,
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Dir(Dir),
    Turn(Turn),
    Forward,
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    val: isize,
    t: Type,
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn dist(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Pos {
    fn move_to(self, dir: &Dir, dist: isize) -> Pos {
        match dir {
            Dir::N => Pos {
                y: self.y + dist,
                ..self
            },
            Dir::S => Pos {
                y: self.y - dist,
                ..self
            },
            Dir::W => Pos {
                x: self.x - dist,
                ..self
            },
            Dir::E => Pos {
                x: self.x + dist,
                ..self
            },
        }
    }

    fn rotate_counter(self) -> Pos {
        Pos {
            x: -self.y,
            y: self.x,
        }
    }

    fn rotate(&self, _around: &Pos, turn: &Turn) -> Pos {
        match turn {
            Turn::L => self.rotate_counter(),
            Turn::R => self.rotate_counter().rotate_counter().rotate_counter(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Ship {
    pos: Pos,
    dir: Dir,
}

impl Ship {
    fn apply(&self, command: &Command) -> Ship {
        let mut s = self.clone();
        match command {
            Command {
                val,
                t: Type::Dir(d),
            } => s.pos = self.pos.move_to(d, *val),
            Command {
                val,
                t: Type::Turn(t),
            } => s.dir = self.dir.turn(t, *val),
            Command {
                val,
                t: Type::Forward,
            } => s.pos = self.pos.move_to(&self.dir, *val),
        };
        s
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            pos: Pos { x: 0, y: 0 },
            dir: Dir::E,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ShipWayPoint {
    ship: Pos,
    waypoint: Pos,
}

impl Default for ShipWayPoint {
    fn default() -> Self {
        Self {
            ship: Pos { x: 0, y: 0 },
            waypoint: Pos { x: 10, y: 1 },
        }
    }
}

impl ShipWayPoint {
    fn apply(&self, command: &Command) -> Self {
        let mut s = self.clone();
        match command {
            Command {
                val,
                t: Type::Dir(d),
            } => {
                s.waypoint = s.waypoint.move_to(d, *val);
            }
            Command {
                val,
                t: Type::Turn(t),
            } => {
                for _ in 0..(val / 90) {
                    s.waypoint = s.waypoint.rotate(&self.ship, t);
                }
            }
            Command {
                val,
                t: Type::Forward,
            } => {
                for _ in 0..*val {
                    s.ship = s.ship + s.waypoint;
                }
            }
        };
        s
    }
}

fn parse(s: &str) -> Command {
    Command {
        val: s[1..].parse().unwrap(),
        t: match s.chars().next() {
            Some('N') => Type::Dir(Dir::N),
            Some('S') => Type::Dir(Dir::S),
            Some('E') => Type::Dir(Dir::E),
            Some('W') => Type::Dir(Dir::W),
            Some('L') => Type::Turn(Turn::L),
            Some('R') => Type::Turn(Turn::R),
            Some('F') => Type::Forward,
            _ => unreachable!(),
        },
    }
}

fn main() {
    let commands: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| parse(&s.unwrap()))
        .collect();

    dbg!(commands
        .iter()
        .fold(Ship::default(), |ship, command| ship.apply(command))
        .pos
        .dist());
    dbg!(commands
        .iter()
        .fold(ShipWayPoint::default(), |ship, command| ship.apply(command))
        .ship
        .dist());
}
