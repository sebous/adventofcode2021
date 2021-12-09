use std::collections::{HashMap, HashSet};

use crate::lib::input::load_input;

type Pattern = HashSet<char>;

#[derive(Debug)]
struct Signal {
    candidates: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

lazy_static! {
    static ref NUMBERS: Vec<Pattern> = {
        [
            vec!['a', 'b', 'c', 'e', 'f', 'g'],
            vec!['c', 'f'],
            vec!['a', 'c', 'd', 'e', 'g'],
            vec!['a', 'c', 'd', 'f', 'g'],
            vec!['b', 'c', 'd', 'f'],
            vec!['a', 'b', 'd', 'f', 'g'],
            vec!['a', 'b', 'd', 'e', 'f', 'g'],
            vec!['a', 'c', 'f'],
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            vec!['a', 'b', 'c', 'd', 'f', 'g'],
        ]
        .iter()
        .map(|chars_vec| HashSet::from_iter(chars_vec.iter().map(|ch| ch.to_owned())))
        .collect()
    };
}

fn parse_input() -> Vec<Signal> {
    let signals = load_input("08")
        .lines()
        .map(|line| {
            let parts = line
                .split(" | ")
                .map(|part| {
                    part.split(" ")
                        .map(|cand| cand.chars().collect::<Pattern>())
                        .collect::<Vec<Pattern>>()
                })
                .collect::<Vec<Vec<Pattern>>>();
            Signal {
                candidates: parts[0].clone(),
                outputs: parts[1].clone(),
            }
        })
        .collect::<Vec<Signal>>();
    signals
}

fn part_one(signals: &Vec<Signal>) -> usize {
    signals
        .iter()
        .flat_map(|sig| &sig.outputs)
        .filter(|output| (output.len() >= 2 && output.len() <= 4) || output.len() == 7)
        .count()
}

fn decode_number_from_pattern(pattern: &Pattern, one: &Pattern, four: &Pattern) -> usize {
    match pattern.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if pattern.intersection(&one).count() == 2 {
                3
            } else if pattern.intersection(&four).count() == 2 {
                2
            } else {
                5
            }
        }
        6 => {
            if pattern.intersection(&one).count() == 1 {
                6
            } else if pattern.intersection(&four).count() == 4 {
                9
            } else {
                0
            }
        }
        7 => 8,
        _ => unreachable!("wrong pattern!!"),
    }
}

fn get_one_and_four(signal: &Signal) -> (Pattern, Pattern) {
    let one = HashSet::from(
        signal
            .candidates
            .iter()
            .find(|pattern| pattern.len() == 2)
            .unwrap()
            .to_owned(),
    );
    let four = HashSet::from(
        signal
            .candidates
            .iter()
            .find(|pattern| pattern.len() == 4)
            .unwrap()
            .to_owned(),
    );
    (one, four)
}

fn part_two(signals: &Vec<Signal>) -> usize {
    let mut output_sum: usize = 0;
    for signal in signals {
        let mut decoded_numbers: HashMap<usize, Pattern> = HashMap::new();
        let (one, four) = get_one_and_four(signal);

        for pattern in &signal.candidates {
            let decoded_num = decode_number_from_pattern(pattern, &one, &four);
            decoded_numbers.insert(decoded_num, pattern.to_owned());
        }

        let output_for_signal = signal
            .outputs
            .iter()
            .fold(String::new(), |acc, pattern| {
                let num_string = decoded_numbers
                    .iter()
                    .find_map(|(key, value)| if value == pattern { Some(key) } else { None })
                    .unwrap()
                    .to_string();

                let mut new_str = String::new();
                new_str.push_str(&acc);
                new_str.push_str(&num_string);
                new_str
            })
            .parse::<usize>()
            .unwrap();

        output_sum += output_for_signal;
    }
    output_sum
}

pub fn run() {
    let input = parse_input();
    let p1 = part_one(&input);
    println!("part 1: {}", p1);
    let p2 = part_two(&input);
    println!("part 2: {}", p2);
}
