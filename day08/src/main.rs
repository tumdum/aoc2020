use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
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

    fn run(&mut self, ops: &[Op]) -> Result<isize, isize> {
        loop {
            if self.pc as usize == ops.len() {
                return Ok(self.acc);
            }
            if !self.seen.insert(self.pc) {
                return Err(self.acc);
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
        .filter_map(|(n, v)| {
            if matches!(v, Op::Acc(_)) {
                None
            } else {
                Some(n)
            }
        })
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
    let _ = dbg!(cpu.run(&input));
    let all_progs = gen_all(&input);
    for p in all_progs {
        let mut cpu = Cpu::new();
        if let Ok(acc) = cpu.run(&p) {
            dbg!(acc);
            break;
        }
    }
}
