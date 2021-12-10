use itertools::Itertools;

use crate::lib::input::load_input;

fn parse_input() -> Vec<Vec<char>> {
    load_input("10")
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect()
}

fn get_oposite_symbol(symbol: &char) -> char {
    match symbol {
        '[' => ']',
        ']' => '[',
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => unreachable!("invalid symbol"),
    }
}

#[derive(Debug)]
struct ProcessResult {
    stack: Vec<char>,
    error: bool,
    err_score: u64,
}

fn process_syntax(tree: &Vec<char>) -> ProcessResult {
    let mut stack = vec![];
    for symbol in tree {
        match symbol {
            '[' | '(' | '{' | '<' => stack.push(*symbol),
            _ => {
                if stack.iter().last().unwrap() == &get_oposite_symbol(symbol) {
                    stack.pop();
                } else {
                    return ProcessResult {
                        stack: stack.clone(),
                        error: true,
                        err_score: match symbol {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!("invalid symbol"),
                        },
                    };
                }
            }
        }
    }
    ProcessResult {
        stack: stack.clone(),
        error: false,
        err_score: 0,
    }
}

fn autocomplete_score(result: &ProcessResult) -> u64 {
    result.stack.iter().rev().fold(0, |acc, symbol| {
        let symbol_score = match get_oposite_symbol(symbol) {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!("nah"),
        };
        acc * 5 + symbol_score
    })
}

fn part_one(input: &Vec<Vec<char>>) -> u64 {
    input
        .iter()
        .map(|line| process_syntax(line))
        .filter(|r| r.error)
        .fold(0, |acc, r| acc + r.err_score)
}

fn part_two(input: &Vec<Vec<char>>) -> u64 {
    let sorted_scores = input
        .iter()
        .map(|l| process_syntax(l))
        .filter(|r| !r.error)
        .map(|r| autocomplete_score(&r))
        .sorted()
        .collect_vec();

    let middle = ((sorted_scores.len() as f64) / 2f64).round();
    sorted_scores[middle as usize - 1]
}

pub fn run() {
    let input = parse_input();
    let p1 = part_one(&input);
    println!("part 1 : {}", p1);

    let p2 = part_two(&input);
    println!("part 2 : {}", p2);
}
