use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

type Deck = VecDeque<usize>;

fn parse(l: &[String]) -> Deck {
    l.iter().skip(1).map(|v| v.parse().unwrap()).collect()
}

fn round(p1: &mut Deck, p2: &mut Deck) {
    let p1card = p1.pop_front().unwrap();
    let p2card = p2.pop_front().unwrap();
    if p1card > p2card {
        p1.push_back(p1card);
        p1.push_back(p2card);
    } else {
        p2.push_back(p2card);
        p2.push_back(p1card);
    }
}

fn combat(mut p1: Deck, mut p2: Deck) -> Deck {
    while !p1.is_empty() && !p2.is_empty() {
        round(&mut p1, &mut p2);
    }
    if p1.is_empty() {
        p2
    } else {
        p1
    }
}

fn final_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| (i + 1) * card)
        .sum()
}

fn recursive_combat(mut p1: Deck, mut p2: Deck) -> (usize, Deck) {
    let mut seen = HashSet::<(Deck, Deck)>::new();
    while !p1.is_empty() && !p2.is_empty() {
        if !seen.insert((p1.clone(), p2.clone())) {
            return (1, p1);
        }
        recursive_round(&mut p1, &mut p2);
    }

    if p1.is_empty() {
        (2, p2)
    } else {
        (1, p1)
    }
}

fn recursive_round(p1: &mut Deck, p2: &mut Deck) {
    let p1card = p1.pop_front().unwrap();
    let p2card = p2.pop_front().unwrap();
    let winner = if p1.len() >= p1card && p2.len() >= p2card {
        recursive_combat(
            p1.iter().take(p1card).cloned().collect(),
            p2.iter().take(p2card).cloned().collect(),
        )
        .0
    } else if p1card > p2card {
        1
    } else {
        2
    };
    if winner == 1 {
        p1.push_back(p1card);
        p1.push_back(p2card);
    } else {
        p2.push_back(p2card);
        p2.push_back(p1card);
    }
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .split(String::is_empty)
        .map(|s| parse(s))
        .collect();
    dbg!(final_score(&combat(input[0].clone(), input[1].clone())));
    dbg!(final_score(
        &recursive_combat(input[0].clone(), input[1].clone()).1
    ));
}
