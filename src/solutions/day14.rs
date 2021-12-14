use std::collections::HashMap;

use itertools::Itertools;

use crate::lib::input::load_input;

type Rules = HashMap<String, char>;

fn parse() -> (Vec<char>, HashMap<String, char>) {
    let input = load_input("14");
    let mut lines = input.lines();
    let template = lines.next().clone().unwrap().chars().collect_vec();

    let rules: HashMap<String, char> = lines
        .skip(1)
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            (from.to_string(), to.chars().next().unwrap())
        })
        .collect();

    (template, rules)
}

fn polymerize(input: &Vec<char>, rules: &Rules) -> Vec<char> {
    let mut result = vec![];
    for i in 0..input.len() - 1 {
        result.push(input[i]);
        let pattern = format!("{}{}", input[i], input[i + 1]);
        if let Some(rule) = rules.get(&pattern) {
            result.push(*rule);
        }
    }
    result.push(input[input.len() - 1]);
    result
}

fn polymerize_recur(
    input: &Vec<char>,
    rules: &Rules,
    buffer: &mut HashMap<char, usize>,
    curr_depth: usize,
    max_depth: usize,
) {
    if curr_depth == max_depth {
        // println!("{}", curr_depth);
        input.iter().for_each(|ch| {
            *buffer.entry(*ch).or_insert(1) += 1;
        });
        return;
    } else {
        let mut new_polymer = input.clone();
        for i in 0..input.len() - 1 {
            new_polymer.push(input[i]);
            let pattern = format!("{}{}", input[i], input[i + 1]);
            if let Some(rule) = rules.get(&pattern) {
                new_polymer.push(*rule);
            }
        }
        new_polymer.push(*input.last().unwrap());

        for i in 0..new_polymer.len() - 1 {
            let pattern = format!("{}{}", new_polymer[i], new_polymer[i + 1])
                .chars()
                .collect_vec();
            polymerize_recur(&pattern, rules, buffer, curr_depth + 1, max_depth);
        }
    }
}

fn part_one(template: &Vec<char>, rules: &Rules) {
    let mut result = template.clone();
    for i in 0..10 {
        println!("{}", i);
        result = polymerize(&result, rules);
    }
    let results_map = result.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(*curr).or_insert(1) += 1;
        acc
    });
    let mut results_sorted = results_map.iter().sorted_by(|a, b| b.1.cmp(a.1));
    println!(
        "{}",
        results_sorted.next().unwrap().1 - results_sorted.last().unwrap().1
    );
}

fn part_two(template: &Vec<char>, rules: &Rules) {
    let mut buffer = HashMap::new();
    polymerize_recur(template, rules, &mut buffer, 0, 5);

    println!("{:?}", buffer);
}

pub fn run() {
    let (template, rules) = parse();
    part_one(&template, &rules);
    // part_two(&template, &rules);
}
