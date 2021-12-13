use std::collections::HashMap;

use itertools::Itertools;

use crate::lib::{
    grid::{Coord, Grid},
    input::load_input,
};

type Instr = (char, usize);

fn parse() -> (Grid<char>, Vec<Instr>) {
    let mut map: HashMap<Coord, char> = HashMap::new();
    let mut instructions: Vec<Instr> = vec![];

    for line in load_input("13").lines() {
        if line.contains(",") {
            let (x, y) = line.split_once(",").unwrap();
            map.insert((x.parse().unwrap(), y.parse().unwrap()), '#');
        } else if line.contains("=") {
            let s = line.split(" ").collect_vec();
            let (axis, value) = s[2].split_once("=").unwrap();
            instructions.push((axis.chars().next().unwrap(), value.parse().unwrap()));
        } else {
            continue;
        }
    }

    let max_x = map.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().clone();
    let max_y = map.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().clone();

    for coords in (0..=max_x.0).cartesian_product(0..=max_y.1) {
        map.entry(coords).or_insert('.');
    }

    (
        Grid {
            map,
            width: max_x.0,
            height: max_y.1,
        },
        instructions,
    )
}

fn fold(grid: &mut Grid<char>, axis: char, value: usize) {
    let folding = grid
        .map
        .keys()
        .filter(|(x, y)| match axis {
            'x' => x > &value,
            'y' => y > &value,
            _ => unreachable!(),
        })
        .cloned()
        .collect_vec();

    for (x, y) in folding {
        let folded_coord = match axis {
            'x' => (value - (x - value), y),
            'y' => (x, value - (y - value)),
            _ => unreachable!(),
        };

        let folding_char = grid.map.get(&(x, y)).unwrap().to_owned();

        if let Some(sym) = grid.map.get_mut(&folded_coord) {
            if *sym == '#' || folding_char == '#' {
                *sym = '#';
            } else {
                *sym = '.';
            }
        }

        grid.map.remove(&(x, y));
    }

    match axis {
        'x' => grid.width = value,
        'y' => grid.height = value,
        _ => unreachable!(),
    }
}

fn part_one(grid: &mut Grid<char>, instructions: &Vec<Instr>) {
    fold(grid, instructions[0].0, instructions[0].1);
    println!(
        "part 1: {}",
        grid.map.values().filter(|sym| *sym == &'#').count()
    );
}

fn part_two(grid: &mut Grid<char>, instructions: &Vec<Instr>) {
    for (axis, value) in instructions {
        fold(grid, *axis, *value);
    }
    println!("{}", grid);
}

pub fn run() {
    let (mut grid, instructions) = parse();
    part_one(&mut grid, &instructions);
    part_two(&mut grid, &instructions);
}
