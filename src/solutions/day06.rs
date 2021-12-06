use std::collections::HashMap;

use crate::lib::input::load_input;

fn parse_input() -> Vec<u8> {
    let input = load_input("06");
    input.split(",").map(|n| n.parse::<u8>().unwrap()).collect()
}

fn fish_generations(input: &Vec<u8>, days: usize) -> usize {
    let mut fish_state: HashMap<u8, usize> = HashMap::new();
    input.iter().for_each(|num| {
        *fish_state.entry(*num).or_insert(0) += 1;
    });

    for _ in 0..days {
        let mut new_generation: HashMap<u8, usize> = HashMap::new();
        for num in fish_state.keys() {
            let count = fish_state.get(num).unwrap();
            match num {
                0 => {
                    *new_generation.entry(6).or_insert(0) += count;
                    *new_generation.entry(8).or_insert(0) += count;
                }
                _ => *new_generation.entry(*num - 1).or_insert(0) += count,
            }
        }
        fish_state = new_generation;
    }
    fish_state.values().sum::<usize>()
}

#[allow(dead_code)]
pub fn run() {
    let input = parse_input();
    let part_one = fish_generations(&input, 80);
    println!("Part 1: {}", part_one);

    let part_two = fish_generations(&input, 256);
    println!("Part 2: {}", part_two);
}
