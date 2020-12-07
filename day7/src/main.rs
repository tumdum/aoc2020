use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::BufRead;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"([^ ]+) ([^ ]+ [^ ]+).*"#).unwrap();
}

fn parse_to(s: &str) -> (usize, String) {
    let c = RE.captures(s).unwrap();
    let n = c[1].parse().unwrap();
    (n, c[2].to_owned())
}

fn parse_line(s: &str) -> (String, Vec<(usize, String)>) {
    let v = s.split(" bags contain ").collect::<Vec<_>>();
    let from = v[0];
    if v[1] == "no other bags." {
        (from.to_owned(), vec![])
    } else {
        (
            from.to_owned(),
            v[1].split(',').map(|s| parse_to(s)).collect::<Vec<_>>(),
        )
    }
}

fn invert_graph(g: &HashMap<String, Vec<(usize, String)>>) -> HashMap<String, HashSet<String>> {
    let mut ret = HashMap::new();
    for (from, to) in g {
        for (_, name) in to {
            ret.entry(name.clone())
                .or_insert(HashSet::new())
                .insert(from.clone());
        }
    }
    ret
}

fn walk_from(g: &HashMap<String, HashSet<String>>, start: &str) -> HashSet<String> {
    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_front(start.to_owned());

    while !todo.is_empty() {
        let next = todo.pop_front().unwrap();
        seen.insert(next.clone());
        if let Some(others) = g.get(&next) {
            for o in others {
                if !seen.contains(o) {
                    todo.push_front(o.clone());
                }
            }
        }
    }
    seen
}

fn bags_in(g: &HashMap<String, Vec<(usize, String)>>, root: &str) -> usize {
    g.get(root)
        .map(|others| {
            others
                .iter()
                .map(|(n, name)| n * bags_in(g, name))
                .sum::<usize>()
                + 1
        })
        .unwrap_or(1)
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse_line(&l.unwrap()))
        .collect::<HashMap<_, _>>();
    dbg!(walk_from(&invert_graph(&input), "shiny gold").len() - 1);
    dbg!(bags_in(&input, "shiny gold") - 1);
}
