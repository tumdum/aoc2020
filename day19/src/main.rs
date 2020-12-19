use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
enum Rule<'a> {
    Lit(&'a str),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

fn parse(s: &str) -> (usize, Rule) {
    let sp = s.split(": ").collect::<Vec<_>>();
    let rules = sp[1];
    (
        sp[0].parse().unwrap(),
        if rules.starts_with('"') {
            Rule::Lit(&rules[1..2])
        } else if rules.contains('|') {
            let v: Vec<Vec<_>> = rules
                .split('|')
                .map(|s| s.trim().split(' ').map(|s| s.parse().unwrap()).collect())
                .collect();
            Rule::Alt(v[0].clone(), v[1].clone())
        } else {
            Rule::Seq(rules.split(' ').map(|s| s.parse().unwrap()).collect())
        },
    )
}

fn matching_tails_seq<'a, 'b>(
    rules: &'a [usize],
    input: &'b str,
    all: &HashMap<usize, Rule>,
) -> Vec<&'b str> {
    rules.iter().fold(vec![input], |v, rule| {
        v.iter()
            .flat_map(|tail| matching_tails(&all[rule], tail, all))
            .collect()
    })
}

fn matching_tails<'a, 'b>(r: &'a Rule, input: &'b str, all: &HashMap<usize, Rule>) -> Vec<&'b str> {
    match r {
        Rule::Lit(s) if input.starts_with(s) => vec![&input[s.len()..]],
        Rule::Lit(_) => vec![],
        Rule::Seq(v) => matching_tails_seq(v, input, all),
        Rule::Alt(a, b) => {
            let mut ret = matching_tails_seq(a, input, all);
            ret.extend(matching_tails_seq(b, input, all));
            ret
        }
    }
}

fn matches(s: &str, all: &HashMap<usize, Rule>) -> bool {
    matching_tails(&all[&0], s, all)
        .into_iter()
        .any(str::is_empty)
}

fn main() {
    let input: Vec<Vec<String>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .split(String::is_empty)
        .map(|s| s.to_vec())
        .collect();
    let mut rules: HashMap<_, _> = input[0].iter().map(|s| parse(s)).collect();
    let input = input[1].clone();

    dbg!(input.iter().filter(|s| matches(s, &rules)).count());

    rules.insert(8, Rule::Alt(vec![42], vec![42, 8]));
    rules.insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));
    dbg!(input.iter().filter(|s| matches(s, &rules)).count());
}

#[test]
fn tests() {
    assert_eq!((3, Rule::Lit("b")), parse("3: \"b\""));
    assert_eq!((0, Rule::Seq(vec![1, 2])), parse("0: 1 2"));
    assert_eq!(
        (2, Rule::Alt(vec![1, 3], vec![3, 1])),
        parse("2: 1 3 | 3 1")
    );
    assert_eq!(
        vec!["b"],
        matching_tails(&Rule::Lit("a"), "ab", &HashMap::new())
    );
    assert!(matching_tails(&Rule::Lit("a"), "bab", &HashMap::new()).is_empty());
}
