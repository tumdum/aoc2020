use std::io::BufRead;

#[derive(Clone, Debug)]
enum Token {
    Num(isize),
    Op(String),
    Open,
    Close,
}

impl Token {
    fn parse(s: &str) -> Token {
        if s == "(" {
            return Token::Open;
        } else if s == ")" {
            return Token::Close;
        }
        if let Ok(v) = s.parse() {
            return Token::Num(v);
        }
        Token::Op(s.to_owned())
    }
}

fn parse(s: &str) -> Vec<Token> {
    let s = s.replace("(", " ( ");
    let s = s.replace(")", " ) ");
    let s = s.replace("*", " * ");
    let s = s.replace("+", " + ");
    let re = regex::Regex::new("([0-9]+|\\+|-|\\*|\\(|\\))+").unwrap();
    re.captures_iter(&s).map(|c| Token::parse(&c[1])).collect()
}

#[derive(Clone, Debug)]
struct State {
    values: Vec<isize>,
    ops: Vec<String>,
}

impl State {
    fn update(&mut self) {
        if self.values.len() > 1 {
            match self.ops.pop() {
                Some(op) => {
                    let rhs = self.values.pop().unwrap();
                    let lhs = self.values.pop().unwrap();
                    let new = match &op as &str {
                        "+" => lhs + rhs,
                        "*" => lhs * rhs,
                        _ => todo!("op: {}", op),
                    };
                    self.values.push(new);
                }
                _ => {}
            }
        }
    }

    fn val(self) -> isize {
        assert!(self.ops.is_empty());
        assert_eq!(1, self.values.len());
        self.values[0]
    }

    fn val2(mut self) -> isize {
        loop {
            let next = match self.ops.iter().enumerate().find(|v| v.1 == "+") {
                Some(v) => (v.0, v.1.clone()),
                None => break,
            };

            let lhs_idx = next.0;
            let rhs_idx = next.0 + 1;
            let lhs = self.values[lhs_idx];
            let rhs = self.values[rhs_idx];
            self.values[lhs_idx] = lhs + rhs;
            self.values.remove(rhs_idx);
            self.ops.remove(next.0);
        }
        self.values.iter().product()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            values: vec![],
            ops: vec![],
        }
    }
}

fn eval(s: &str) -> isize {
    let e = parse(s);
    let mut states = vec![State::default()];
    for e in e {
        match e {
            Token::Num(v) => states.last_mut().unwrap().values.push(v),
            Token::Op(op) => states.last_mut().unwrap().ops.push(op),
            Token::Open => states.push(State::default()),
            Token::Close => {
                let val = states.pop().unwrap().val();
                states.last_mut().unwrap().values.push(val);
            },
        }
        states.last_mut().unwrap().update();
    }
    states.pop().unwrap().val()
}

fn eval2(s: &str) -> isize {
    let e = parse(s);
    let mut states = vec![State::default()];
    for e in e {
        match e {
            Token::Num(v) => states.last_mut().unwrap().values.push(v),
            Token::Op(op) => states.last_mut().unwrap().ops.push(op),
            Token::Open => states.push(State::default()),
            Token::Close => {
                let val = states.pop().unwrap().val2();
                states.last_mut().unwrap().values.push(val);
            },
        }
    }
    states.pop().unwrap().val2()
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect();

    dbg!(input.iter().map(|s| eval(s)).sum::<isize>());
    dbg!(input.iter().map(|s| eval2(s)).sum::<isize>());
}

#[test]
fn eval_tests() {
    assert_eq!(71, eval("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(51, eval("1 + (2 * 3) + (4 * (5 + 6))"));
    assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    assert_eq!(13632, eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));

    assert_eq!(231, eval2("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(46, eval2("2 * 3 + (4 * 5)"));
    assert_eq!(1445, eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    assert_eq!(669060, eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
}
