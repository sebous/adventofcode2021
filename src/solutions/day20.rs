use std::collections::HashMap;

use itertools::Itertools;

use crate::lib::{
    grid::{Coord, Grid},
    input::load_input,
};

const PADDING: usize = 1000;

fn parse() -> (Grid<u8>, Vec<u8>) {
    let input = load_input("20");
    let alg = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => unreachable!(),
        })
        .collect_vec();

    let width = input.lines().skip(2).next().unwrap().chars().count();
    let height = input.lines().skip(2).count();

    let mut map = HashMap::new();

    for (y, line) in input.lines().skip(2).enumerate() {
        for (x, n) in line
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => unreachable!(),
            })
            .enumerate()
        {
            map.insert((x + PADDING, y + PADDING), n);
        }
    }

    (Grid { map, width, height }, alg)
}

pub fn get_all_adjacent(&(x, y): &Coord) -> impl Iterator<Item = Coord> {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
}

fn pass(input: &Grid<u8>, alg: &Vec<u8>, default: char) -> Grid<u8> {
    let (min_x, min_y) = input.get_min_coord();
    let mut output_map = HashMap::new();

    for c in (min_x - 1..=input.width + min_x).cartesian_product(min_y - 1..=input.height + min_y) {
        let b_code = get_all_adjacent(&c)
            .map(|adj_c| match input.map.get(&adj_c) {
                Some(1) => '1',
                Some(0) => '0',
                _ => default,
            })
            .collect::<String>();

        let dec_val = usize::from_str_radix(&b_code, 2).unwrap();
        let value = alg[dec_val];

        output_map.insert(c, value);
    }

    Grid {
        height: input.height + 2,
        width: input.height + 2,
        map: output_map,
    }
}

fn part_one(grid: &Grid<u8>, alg: &Vec<u8>) {
    let mut grid = grid.clone();
    let mut default = '0';
    for i in 0..50 {
        grid = pass(&grid, alg, default);

        if i == 1 {
            println!(
                "part 1: {}",
                grid.map.values().filter(|v| *v == &1u8).count()
            );
        }

        default = if default == '0' { '1' } else { '0' };
    }
    println!(
        "part 2: {}",
        grid.map.values().filter(|v| *v == &1u8).count()
    );
}

pub fn run() {
    let (grid, alg) = parse();
    part_one(&grid, &alg);
}
