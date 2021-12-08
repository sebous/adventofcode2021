use std::collections::HashMap;

use crate::lib::input::load_input;

type Pattern = Vec<char>;

#[derive(Debug)]
struct Signal {
    candidates: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

const NUMBERS: &'static [&'static [&'static char]] = &[&[&'a', &'b', &'c']];

fn get_numbers() -> HashMap<usize, Pattern> {
    vec![
        // 0
        vec!['a', 'b', 'c', 'e', 'f', 'g'],
        // 1
        vec!['c', 'f'],
        // 2
        vec!['a', 'c', 'd', 'e', 'g'],
        // 3
        vec!['a', 'c', 'd', 'f', 'g'],
        // 4
        vec!['b', 'c', 'd', 'f'],
        // 5
        vec!['a', 'b', 'd', 'f', 'g'],
        // 6
        vec!['a', 'b', 'd', 'e', 'f', 'g'],
        // 7
        vec!['a', 'c', 'f'],
        // 8
        vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        // 9
        vec!['a', 'b', 'c', 'd', 'f', 'g'],
    ]
    .iter()
    .enumerate()
    .map(|(i, pattern)| (i, pattern.to_owned()))
    .collect()
}

fn parse_input() -> Vec<Signal> {
    let signals = load_input("08")
        .lines()
        .map(|line| {
            let parts = line
                .split(" | ")
                .map(|part| {
                    part.split(" ")
                        .map(|cand| cand.chars().collect::<Vec<char>>())
                        .collect::<Vec<Pattern>>()
                })
                .collect::<Vec<Vec<Pattern>>>();

            let mut candidates = parts[0].clone();
            candidates.sort_by(|a, b| a.len().cmp(&b.len()));
            let mut outputs = parts[1].clone();
            outputs.sort_by(|a, b| a.len().cmp(&b.len()));

            Signal {
                candidates,
                outputs,
            }
        })
        .collect::<Vec<Signal>>();
    signals
}

fn part_one(signals: &Vec<Signal>) -> usize {
    signals
        .iter()
        .map(|sig| &sig.outputs)
        .flatten()
        .filter(|output| (output.len() >= 2 && output.len() <= 4) || output.len() == 7)
        .count()
}

fn part_two(signals: &Vec<Signal>) {
    // let probabilities = HashMap::new();
    // let numbers = get_numbers();

    // let example_candidates = signals[0].candidates;
    // for candidate in &example_candidates {
    //     let possible_numbers = numbers
    //         .iter()
    //         .filter(|(num, chars)| chars.len() == candidate.len())
    //         .collect::<Vec<char>>();
    // }
}

pub fn run() {
    let input = parse_input();
    let p1 = part_one(&input);
    println!("part 1: {}", p1);
}
