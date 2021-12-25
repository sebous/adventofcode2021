use std::collections::HashMap;

use crate::lib::{grid::Grid, input::load_input};

fn parse() -> Grid<char> {
    let input = load_input("25");
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert((x, y), ch);
        }
    }

    Grid { height, width, map }
}

fn step(grid: &mut Grid<char>) -> usize {
    let mut next_map = grid.map.clone();
    let mut counter = 0;

    // east
    for (coord, _) in grid.map.iter().filter(|(_, &val)| val == '>') {
        let (next_c, next) = if let Some(n) = grid.map.get_key_value(&(coord.0 + 1, coord.1)) {
            n
        } else {
            grid.map.get_key_value(&(0, coord.1)).unwrap()
        };

        if *next == '.' {
            next_map.insert(*next_c, '>');
            next_map.insert(*coord, '.');
            counter += 1;
        }
    }

    let map = next_map;
    let mut next_map = map.clone();

    // south
    for (coord, _) in map.iter().filter(|(_, &val)| val == 'v') {
        let (next_c, next) = if let Some(n) = map.get_key_value(&(coord.0, coord.1 + 1)) {
            n
        } else {
            map.get_key_value(&(coord.0, 0)).unwrap()
        };

        if *next == '.' {
            next_map.insert(*next_c, 'v');
            next_map.insert(*coord, '.');
            counter += 1;
        }
    }

    grid.map = next_map;
    counter
}

fn part_one(grid: &Grid<char>) {
    let mut grid = grid.clone();
    let mut i = 0;
    loop {
        i += 1;
        let moved_count = step(&mut grid);
        if moved_count == 0 {
            break;
        }
    }
    println!("part 1 :{}", i);
}

pub fn run() {
    let grid = parse();
    part_one(&grid);
}
