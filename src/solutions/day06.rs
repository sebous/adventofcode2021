use std::collections::HashMap;

use crate::lib::input::load_input;

fn parse_input() -> Vec<u8> {
    let input = load_input("06");
    input.split(",").map(|n| n.parse::<u8>().unwrap()).collect()
}

#[allow(dead_code)]
fn fish_generations_v2(input: &Vec<u8>, days: usize) -> usize {
    let mut fish = [0; 9];
    input.iter().for_each(|n| fish[*n as usize] += 1);

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }
    fish.iter().sum()
}

fn fish_generations(input: &Vec<u8>, days: usize) -> usize {
    let mut generation = HashMap::new();
    input.iter().for_each(|num| {
        *generation.entry(*num).or_insert(0) += 1;
    });

    for _ in 0..days {
        let mut new_generation = HashMap::new();
        for (num, count) in generation.iter() {
            match num {
                0 => {
                    *new_generation.entry(6).or_insert(0) += count;
                    *new_generation.entry(8).or_insert(0) += count;
                }
                _ => *new_generation.entry(*num - 1).or_insert(0) += count,
            }
        }
        generation = new_generation;
    }
    generation.values().sum()
}

#[allow(dead_code)]
pub fn run() {
    let input = parse_input();
    let part_one = fish_generations(&input, 80);
    println!("Part 1: {}", part_one);
    let part_two = fish_generations(&input, 256);
    println!("Part 2: {}", part_two);
}
