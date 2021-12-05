use std::collections::HashMap;

use regex::Regex;

use crate::lib::input::load_input;
use itertools::*;

type Coord = (usize, usize);
type Path = (Coord, Coord);

fn parse_input() -> Vec<Path> {
    let input = load_input("05");
    let coord_regex = Regex::new(r"(\d+,\d+)").unwrap();
    let lines: Vec<Path> = input
        .lines()
        .map(|l| {
            let mut coords = coord_regex
                .captures_iter(l)
                .map(|c| c.get(0).unwrap().as_str().to_owned())
                .map(|coord_str| {
                    coord_str
                        .split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .next_tuple()
                        .unwrap()
                });

            (coords.next().unwrap(), coords.next().unwrap())
        })
        .collect();

    lines
}

fn write_coords_to_grid(coords: &Vec<Coord>, grid: &mut HashMap<Coord, usize>) {
    for coord in coords.iter() {
        if let Some(point) = grid.get_mut(coord) {
            *point += 1;
        } else {
            grid.insert(*coord, 1);
        }
    }
}

#[derive(Debug)]
enum Direction {
    HORIZONTAL,
    VERTICAL,
    DIAGONAL,
}

impl Direction {
    fn get_variant(((x1, y1), (x2, y2)): &Path) -> Self {
        if y1 == y2 && x1 != x2 {
            Direction::HORIZONTAL
        } else if x1 == x2 && y1 != y2 {
            Direction::VERTICAL
        } else if (*x1 as i32 - *x2 as i32).abs() == (*y1 as i32 - *y2 as i32).abs() {
            Direction::DIAGONAL
        } else {
            panic!("not implemented line direction found!");
        }
    }
}

fn get_coords_on_path(path: &Path) -> Vec<Coord> {
    let ((x1, y1), (x2, y2)) = path;
    let direction: Direction = Direction::get_variant(path);

    let (low_x, high_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (low_y, high_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let range: Vec<Coord> = match direction {
        Direction::HORIZONTAL => (*low_x..*high_x + 1).map(|num| (num, *y1)).collect(),
        Direction::VERTICAL => (*low_y..*high_y + 1).map(|num| (*low_x, num)).collect(),
        Direction::DIAGONAL => {
            if x1 < x2 {
                (*x1..*x2 + 1)
                    .enumerate()
                    .map(|(i, x)| (x, if y1 < y2 { y1 + i } else { y1 - i }))
                    .collect()
            } else {
                (*x2..*x1 + 1)
                    .enumerate()
                    .map(|(i, x)| (x, if y2 < y1 { y2 + i } else { y2 - i }))
                    .collect()
            }
        }
    };
    range
}

fn part_one(lines: &Vec<Path>) -> usize {
    let mut grid: HashMap<Coord, usize> = HashMap::new();

    lines
        .into_iter()
        .filter(|path| match Direction::get_variant(path) {
            Direction::HORIZONTAL => true,
            Direction::VERTICAL => true,
            _ => false,
        })
        .for_each(|path| {
            let coords_on_path = get_coords_on_path(path);
            write_coords_to_grid(&coords_on_path, &mut grid);
        });

    grid.iter().filter(|(_, val)| *val >= &2).count()
}

fn part_two(lines: &Vec<(Coord, Coord)>) -> usize {
    let mut grid: HashMap<Coord, usize> = HashMap::new();

    lines.into_iter().for_each(|path| {
        let coords_on_path = get_coords_on_path(path);
        write_coords_to_grid(&coords_on_path, &mut grid);
    });

    grid.iter().filter(|(_, val)| *val >= &2).count()
}

#[allow(dead_code)]
pub fn run() {
    let lines = parse_input();
    let p1 = part_one(&lines);
    println!("Part 1: {}", p1);

    let p2 = part_two(&lines);
    println!("Part 2: {}", p2);
}
