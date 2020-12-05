use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"([^ ]+):([^ ]+)"#).unwrap();
    static ref HCL: Regex = Regex::new(r#"#[0-9a-f]{6}"#).unwrap();
    static ref PID: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
    static ref ECL: HashSet<&'static str> =
        maplit::hashset! {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
}

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse_line(l: &str) -> HashMap<String, String> {
    RE.captures_iter(l)
        .map(|c| (c[1].to_owned(), c[2].to_owned()))
        .collect()
}

fn merge(l: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    l.split(HashMap::is_empty)
        .map(|maps| {
            maps.iter().fold(HashMap::new(), |acc, m| {
                acc.into_iter().chain(m.clone()).collect()
            })
        })
        .collect()
}

fn has_needed(m: &&HashMap<String, String>) -> bool {
    FIELDS.iter().all(|e| m.contains_key(*e))
}

fn valid_num(s: &str, from: u32, to: u32) -> bool {
    s.parse::<u32>().map_or(false, |v| (from..=to).contains(&v))
}

fn valid_hgt(s: &str) -> bool {
    if s.ends_with("cm") {
        valid_num(&s[..s.len() - 2], 150, 193)
    } else if s.ends_with("in") {
        valid_num(&s[..s.len() - 2], 59, 76)
    } else {
        false
    }
}

fn all_valid(m: &HashMap<String, String>) -> bool {
    valid_num(&m["byr"], 1920, 2002)
        && valid_num(&m["iyr"], 2010, 2020)
        && valid_num(&m["eyr"], 2020, 2030)
        && valid_hgt(&m["hgt"])
        && ECL.contains(&m["ecl"] as &str)
        && HCL.is_match(&m["hcl"])
        && PID.is_match(&m["pid"])
}

fn main() {
    let input = merge(
        std::io::stdin()
            .lock()
            .lines()
            .map(|l| parse_line(&l.unwrap()))
            .collect::<Vec<_>>(),
    );
    dbg!(input.iter().filter(has_needed).count());
    dbg!(input
        .iter()
        .filter(|v| has_needed(v) && all_valid(v))
        .count());
}
