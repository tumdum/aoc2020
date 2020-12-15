use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static!{
    static ref RE: Regex = Regex::new(r#"^(.+)-(.+) (.): (.+)$"#).unwrap();
}

#[derive(Debug)]
struct Rule {
    from: usize,
    to: usize,
    letter: char,
}

impl Rule {
    fn check(&self, pass: &str) -> bool {
        (self.from..=self.to).contains(&pass.chars().filter(|c| *c == self.letter).count())
    }
    fn check2(&self, pass: &str) -> bool {
        let c : Vec<_> = pass.chars().collect();
        (c[self.from-1] == self.letter) ^ (c[self.to-1] == self.letter)
    }
}

fn parse(l: &str) -> (Rule, String) {
    let c = RE.captures(l).unwrap();
    let from : usize = c[1].parse().unwrap();
    let to : usize = c[2].parse().unwrap();
    let letter = c[3].chars().next().unwrap();
    let pass = c[4].to_string();
    (Rule{from, to, letter}, pass)
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse(&l.unwrap()))
        .collect();
    println!("{}", input.iter().filter(|(r, p)| r.check(p)).count());
    println!("{}", input.iter().filter(|(r, p)| r.check2(p)).count());
}
