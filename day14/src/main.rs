use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const M: u64 = 0b00000000_00000000_00000000_00001111_11111111_11111111_11111111_11111111;

#[derive(Debug, Clone)]
struct Mask {
    bits: HashMap<u64, bool>,
    floating: HashSet<u64>,
}

fn set_bit(v: u64, bit: u64) -> u64 {
    v | 1 << bit
}

fn unset_bit(v: u64, bit: u64) -> u64 {
    let b: u64 = M & !(1 << bit);
    v & b
}

impl Mask {
    fn new(s: &str) -> Self {
        assert_eq!(36, s.len());
        Self {
            bits: s
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != 'X')
                .map(|(n, c)| (35 - n as u64, c == '1'))
                .collect(),
            floating: s
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(|(n, _)| 35 - n as u64)
                .collect(),
        }
    }

    fn apply(&self, mut v: u64) -> u64 {
        for (n, b) in &self.bits {
            if *b {
                v = set_bit(v, *n);
            } else {
                v = unset_bit(v, *n);
            }
        }
        v
    }

    fn apply_addr(&self, mut v: u64) -> Vec<u64> {
        for (n, b) in &self.bits {
            if *b {
                v = set_bit(v, *n);
            }
        }
        self.floating.iter().fold(vec![v], |acc, n| {
            acc.iter()
                .flat_map(|v| vec![set_bit(*v, *n), unset_bit(*v, *n)])
                .collect::<Vec<_>>()
        })
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            bits: Default::default(),
            floating: Default::default(),
        }
    }
}

#[derive(Debug)]
enum Op {
    Mask(Mask),
    Write { add: u64, val: u64 },
}

fn parse(s: &str) -> Op {
    if s.starts_with("mask = ") {
        Op::Mask(Mask::new(&s[7..]))
    } else {
        let re = Regex::new(r#"^mem\[(.+)\] = (.+)$"#).unwrap();
        let c = re.captures(s).unwrap();
        Op::Write {
            add: c[1].parse().unwrap(),
            val: c[2].parse().unwrap(),
        }
    }
}

fn init(p: &[Op]) -> HashMap<u64, u64> {
    let mut m = HashMap::new();
    let mut mask = Mask::default();
    for op in p {
        match op {
            Op::Mask(m) => mask = m.clone(),
            Op::Write { add, val } => *m.entry(*add).or_insert(0) = mask.apply(*val),
        }
    }
    m
}

fn init2(p: &[Op]) -> HashMap<u64, u64> {
    let mut m = HashMap::new();
    let mut mask = Mask::default();
    for op in p {
        match op {
            Op::Mask(m) => mask = m.clone(),
            Op::Write { add, val } => {
                let addresses = mask.apply_addr(*add);
                for a in addresses {
                    *m.entry(a).or_insert(0) = *val;
                }
            }
        }
    }
    m
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| parse(&s.unwrap()))
        .collect();
    dbg!(init(&input).values().sum::<u64>());
    dbg!(init2(&input).values().sum::<u64>());
}

#[test]
fn mask() {
    assert_eq!(
        73,
        Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(11)
    );
    assert_eq!(
        101,
        Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(101)
    );
    assert_eq!(
        64,
        Mask::new("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").apply(0)
    );
}
