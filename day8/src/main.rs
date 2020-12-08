use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Op {
    fn is_acc(&self) -> bool {
        if let Op::Acc(_) = self {
            true
        } else {
            false
        }
    }
}
fn parse_op(s: &str) -> Op {
    let v: Vec<_> = s.split(' ').collect();
    let arg = v[1].parse::<isize>().unwrap();
    match v[0] {
        "acc" => Op::Acc(arg),
        "jmp" => Op::Jmp(arg),
        "nop" => Op::Nop(arg),
        _ => unimplemented!(),
    }
}

struct Cpu {
    pc: isize,
    acc: isize,
    seen: HashSet<isize>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            pc: 0,
            acc: 0,
            seen: Default::default(),
        }
    }

    fn run(&mut self, ops: &[Op]) -> bool {
        loop {
            if self.pc as usize == ops.len() {
                return true;
            }
            if !self.seen.insert(self.pc) {
                return false;
            }
            match ops[self.pc as usize] {
                Op::Acc(v) => {
                    self.acc += v;
                    self.pc += 1;
                }
                Op::Jmp(v) => {
                    self.pc += v;
                }
                Op::Nop(_) => {
                    self.pc += 1;
                }
            }
        }
    }
}

fn change(op: &Op) -> Op {
    match op {
        Op::Jmp(v) => Op::Nop(*v),
        Op::Nop(v) => Op::Jmp(*v),
        Op::Acc(v) => Op::Acc(*v),
    }
}

fn gen_all(input: &[Op]) -> Vec<Vec<Op>> {
    input
        .iter()
        .enumerate()
        .filter_map(|(n, v)| if !v.is_acc() { Some(n) } else { None })
        .map(|i| {
            let mut new = input.to_vec();
            new[i] = change(&input[i]);
            new
        })
        .collect()
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| parse_op(&s.unwrap()))
        .collect();
    let mut cpu = Cpu::new();
    cpu.run(&input);
    dbg!(cpu.acc);
    let all_progs = gen_all(&input);
    for p in all_progs {
        let mut cpu = Cpu::new();
        let result = cpu.run(&p);
        if result {
            dbg!(cpu.acc);
            break;
        }
    }
}
