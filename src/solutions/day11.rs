use std::{collections::HashMap, fmt};

use itertools::Itertools;

use crate::lib::{
    grid::{Coord, Grid},
    input::load_input,
};

#[derive(Debug, Clone)]
struct OctopusState {
    value: usize,
    flashed: bool,
}

impl fmt::Display for OctopusState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Grid<OctopusState> {
    fn init_round(&mut self) {
        self.map.values_mut().for_each(|octopus| {
            octopus.value += 1;
            octopus.flashed = false;
        });
    }

    fn flash_pass(&self) -> (Vec<Coord>, HashMap<Coord, usize>) {
        let mut affected_octs = HashMap::new();

        let flashing = self
            .map
            .iter()
            .filter(|(_, oct)| !oct.flashed && oct.value > 9);

        flashing.clone().for_each(|(coord, _)| {
            self.get_adjacent_coords(&coord, true)
                .filter(|coord| !self.map.get(coord).unwrap().flashed)
                .for_each(|coord| *affected_octs.entry(coord).or_insert(0) += 1)
        });

        let flashed = flashing.map(|(coord, _)| coord.to_owned()).collect_vec();

        (flashed, affected_octs)
    }

    fn step(&mut self) -> usize {
        self.init_round();
        let mut flashes_count = 0;

        loop {
            let (flashed, affected_octs) = self.flash_pass();
            if flashed.len() == 0 {
                break;
            }
            flashes_count += flashed.len();

            for coord in self.iter_coords() {
                if flashed.contains(&coord) {
                    if let Some(oct) = self.map.get_mut(&coord) {
                        oct.flashed = true;
                        oct.value = 0;
                    }
                }

                if affected_octs.keys().contains(&coord) {
                    if let Some(oct) = self.map.get_mut(&coord) {
                        if !oct.flashed {
                            oct.value += affected_octs.get(&coord).unwrap();
                        }
                    }
                }
            }
        }
        flashes_count
    }
}

fn parse_input() -> Grid<OctopusState> {
    let mut map: HashMap<Coord, OctopusState> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in load_input("11").lines().enumerate() {
        for (x, val) in line.chars().enumerate() {
            map.insert(
                (x, y),
                OctopusState {
                    value: val.to_digit(10).unwrap() as usize,
                    flashed: false,
                },
            );
        }
        if y == 0 {
            width = line.len();
        }
        height += 1;
    }

    Grid { map, width, height }
}

fn part_one(grid: &Grid<OctopusState>) -> usize {
    let mut grid = grid.clone();
    (0..100).map(|_| grid.step()).sum()
}

fn part_two(grid: &Grid<OctopusState>) -> usize {
    let mut grid = grid.clone();
    let oct_count = grid.height * grid.width;
    let mut i = 1;

    loop {
        let flashes = grid.step();
        if flashes == oct_count {
            break;
        }
        i += 1;
    }
    i
}

pub fn run() {
    let grid = parse_input();
    let p1 = part_one(&grid);
    println!("part 1: {}", p1);
    let p2 = part_two(&grid);
    println!("part 2: {}", p2);
}
