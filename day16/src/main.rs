use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::RangeInclusive;

fn parse(s: &str) -> Vec<usize> {
    s.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

fn parse_range(s: &str) -> RangeInclusive<usize> {
    let mut s = s.split('-').map(|s| s.parse().unwrap());
    RangeInclusive::new(s.next().unwrap(), s.next().unwrap())
}

fn parse_rule(sss: &str) -> (String, Rule) {
    let ss: Vec<_> = sss.split(':').collect();
    let s: Vec<_> = ss[1].split(' ').collect();
    let a = parse_range(s[1]);
    let b = parse_range(s[3]);
    (ss[0].to_owned(), Rule { rules: vec![a, b] })
}

#[derive(Debug, Clone)]
struct Rule {
    rules: Vec<RangeInclusive<usize>>,
}

impl Rule {
    fn matches(&self, v: usize) -> bool {
        self.rules.iter().any(|r| r.contains(&v))
    }
}

fn split(input: &[String]) -> (HashMap<String, Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let mut input = input.split(String::is_empty);
    let rules = input
        .next()
        .unwrap()
        .iter()
        .map(|s| parse_rule(&s))
        .collect::<HashMap<_, _>>();
    let ticket = parse(&input.next().unwrap()[1]);
    let nearby = input.next().unwrap()[1..]
        .iter()
        .map(|s| parse(&s))
        .collect::<Vec<_>>();
    (rules, ticket, nearby)
}

fn find_not_matching(rules: &HashMap<String, Rule>, ticket: &[usize]) -> Vec<usize> {
    ticket
        .iter()
        .filter(|v| !rules.values().any(|r| r.matches(**v)))
        .cloned()
        .collect()
}

fn find_possible_idx_names(
    tickets: &[Vec<usize>],
    rules: &HashMap<String, Rule>,
    idx: usize,
) -> HashSet<String> {
    let ticket_values: Vec<usize> = tickets.iter().map(|t| t[idx]).collect();
    rules
        .iter()
        .filter(|(_, r)| ticket_values.iter().all(|v| r.matches(*v)))
        .map(|(name, _)| name.clone())
        .collect()
}

fn main() {
    let (rules, ticket, nearby) = split(
        &std::io::stdin()
            .lock()
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>(),
    );

    dbg!(nearby
        .iter()
        .flat_map(|t| find_not_matching(&rules, t))
        .sum::<usize>());

    let mut valid: Vec<Vec<usize>> = nearby
        .iter()
        .filter(|t| find_not_matching(&rules, t).is_empty())
        .cloned()
        .collect();
    valid.push(ticket.clone());
    let mut possible_names_of_idx = (0..valid[0].len())
        .map(|idx| (idx, find_possible_idx_names(&valid, &rules, idx)))
        .collect::<Vec<_>>();
    possible_names_of_idx.sort_by_key(|v| v.1.len());
    let mut name_to_idx = HashMap::<String, usize>::new();
    for (idx, mut names) in possible_names_of_idx {
        for n in name_to_idx.keys() {
            names.remove(n);
        }
        assert_eq!(1, names.len());
        name_to_idx.insert(names.iter().next().unwrap().clone(), idx);
    }
    let departure: Vec<_> = name_to_idx
        .into_iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| idx)
        .collect();
    dbg!(departure
        .into_iter()
        .map(|idx| ticket[idx])
        .product::<usize>());
}
