use std::collections::HashMap;

use crate::lib::input::load_input;

fn parse_input() -> Vec<usize> {
    load_input("07")
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn distance(a: usize, b: usize) -> usize {
    (std::cmp::min(a, b)..=std::cmp::max(a, b))
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, _)| acc + (i))
}

fn median(input: &Vec<usize>) -> usize {
    let mut list = input.clone();
    list.sort();
    list[list.len() / 2]
}

fn part_two(input: &Vec<usize>) -> usize {
    let max = input.iter().max().unwrap();
    let fuel = |num| input.iter().fold(0, |acc, curr| acc + distance(num, *curr));

    let median = median(input);
    let mut results = HashMap::new();

    for num in median..*max {
        let curr_fuel = fuel(num);
        results.insert(num, curr_fuel);
        if let Some(x) = results.get(&(num - 1)) {
            if curr_fuel > *x {
                return *x;
            }
        }
    }
    panic!("oh noooo");
}

fn part_one(input: &Vec<usize>) -> usize {
    let median = median(input);
    input.iter().fold(0, |acc, num| {
        acc + (median as i32 - *num as i32).abs() as usize
    })
}

pub fn run() {
    let input = parse_input();
    let p1 = part_one(&input);
    println!("part 1: {}", p1);

    let p2 = part_two(&input);
    println!("part 2: {}", p2);
}

#[test]
fn distance_test() {
    assert_eq!(distance(16, 5), 66);
    assert_eq!(distance(0, 5), 15);
}
