use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

use crate::lib::input::load_input;

type Template = BTreeMap<(char, char), usize>;
type Rules = BTreeMap<(char, char), char>;

fn parse() -> (Template, Rules) {
    let input = load_input("14");
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().chars().collect_vec();

    let template = (0..first_line.len() - 1)
        .map(|i| ((first_line[i], first_line[i + 1]), 1usize))
        .collect();

    let rules = lines
        .skip(1)
        .map(|line| {
            let mut line = line.chars();

            (
                (line.next().unwrap(), line.next().unwrap()),
                line.last().unwrap(),
            )
        })
        .collect();
    (template, rules)
}

fn polymerize(template: &Template, rules: &Rules) -> Template {
    let mut result = BTreeMap::new();

    for (&pair @ (a, b), n) in template {
        if let Some(&c) = rules.get(&pair) {
            *result.entry((a, c)).or_insert(0) += n;
            *result.entry((c, b)).or_insert(0) += n;
        } else {
            *result.entry(pair).or_insert(0) += n;
        }
    }
    result
}

fn count_difference(template: &Template) -> usize {
    let mut items = BTreeMap::new();
    for (&(a, b), &count) in template {
        *items.entry(a).or_insert(0) += count;
        *items.entry(b).or_insert(0) += count;
    }
    (items.values().max().unwrap() + 1) / 2 - (items.values().min().unwrap() + 1) / 2
}

fn part_one() -> usize {
    let (template, rules) = parse();
    let result = (0..2).fold(template, |acc, _| polymerize(&acc, &rules));
    count_difference(&result)
}

fn part_two() -> usize {
    let (template, rules) = parse();
    let result = (0..40).fold(template, |acc, _| polymerize(&acc, &rules));
    count_difference(&result)
}

pub fn run() {
    let p1 = part_one();
    println!("part 1: {}", p1);

    let p2 = part_two();
    println!("part 2: {}", p2);
}
