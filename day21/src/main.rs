use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn parse(s: &str) -> (HashSet<String>, HashSet<String>) {
    let mut s = s.split(" (contains ");
    let ingredients = s.next().unwrap().split(' ').map(|s| s.to_owned()).collect();
    let allergens = s.next().unwrap();
    let allergens: HashSet<String> = allergens[..allergens.len() - 1]
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    (ingredients, allergens)
}

fn find_uniq(all: &HashMap<String, HashSet<String>>) -> Option<(String, String)> {
    all.iter()
        .find(|v| v.1.len() == 1)
        .map(|v| (v.0.clone(), v.1.iter().next().unwrap().clone()))
}

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| parse(&l.unwrap()))
        .collect();
    let mut allergens_to_ingredients: HashMap<String, Vec<HashSet<String>>> = HashMap::new();
    for (ingredients, allergens) in &input {
        for allergen in allergens {
            allergens_to_ingredients
                .entry(allergen.clone())
                .or_default()
                .push(ingredients.clone());
        }
    }
    let mut allergens_to_ingredients: HashMap<String, HashSet<String>> = allergens_to_ingredients
        .into_iter()
        .map(|(allergen, ingredients_list)| {
            (
                allergen,
                ingredients_list
                    .iter()
                    .fold(ingredients_list[0].clone(), |acc, new| {
                        acc.intersection(new).cloned().collect()
                    }),
            )
        })
        .collect();
    let all: HashSet<String> = input.iter().flat_map(|v| v.0.clone()).collect();
    let candidates: HashSet<String> = allergens_to_ingredients
        .values()
        .flatten()
        .cloned()
        .collect();
    let safe: HashSet<String> = all.difference(&candidates).cloned().collect();
    dbg!(input
        .iter()
        .map(|v| v.0.intersection(&safe).count())
        .sum::<usize>());

    // part 2
    let mut mapped: Vec<(String, String)> = vec![];
    loop {
        if let Some((allergen, ingredient)) = find_uniq(&allergens_to_ingredients) {
            allergens_to_ingredients.remove(&allergen);
            for (_, ingredients) in allergens_to_ingredients.iter_mut() {
                ingredients.remove(&ingredient);
            }
            mapped.push((ingredient, allergen));
        } else {
            break;
        }
    }
    mapped.sort_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs));
    let mapped: Vec<_> = mapped
        .into_iter()
        .map(|(ingredient, _)| ingredient)
        .collect();
    dbg!(mapped.join(","));
}
