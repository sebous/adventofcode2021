use std::collections::HashMap;

use itertools::Itertools;

use crate::lib::input::load_input;

type Coord = (usize, usize);
type Location = (Coord, u32);
type Grid = HashMap<Coord, u32>;
struct GridInfo {
    grid: Grid,
    size_x: usize,
    size_y: usize,
}

fn parse_input() -> GridInfo {
    let mut grid = HashMap::new();
    let input = load_input("09");
    let mut size_x = 0;
    let size_y = input.lines().into_iter().count();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.insert((x, y), char.to_digit(10).unwrap());
        }
        if size_x == 0 {
            size_x = line.chars().count();
        }
    }
    GridInfo {
        grid,
        size_x,
        size_y,
    }
}

fn get_adjacent((x, y): Coord, grid_info: &GridInfo) -> Vec<Location> {
    let adjacent_coords: Vec<(isize, isize)> = vec![
        (x as isize - 1, y as isize),
        (x as isize + 1, y as isize),
        (x as isize, y as isize - 1),
        (x as isize, y as isize + 1),
    ];
    let adjacent_coords = adjacent_coords
        .iter()
        .filter(|(x, y)| {
            *x >= 0 && *x < grid_info.size_x as isize && *y >= 0 && *y < grid_info.size_y as isize
        })
        .map(|(x, y)| (x.to_owned() as usize, y.to_owned() as usize))
        .collect::<Vec<Coord>>();

    grid_info
        .grid
        .iter()
        .filter(|(coord, _)| adjacent_coords.contains(coord))
        .map(|(coord, num)| (coord.to_owned(), num.to_owned()))
        .collect()
}

fn get_low_points(grid_info: &GridInfo) -> Vec<Location> {
    let mut low_points = vec![];

    for y in 0..grid_info.size_y {
        for x in 0..grid_info.size_x {
            if let Some(current) = grid_info.grid.get(&(x, y)) {
                let adjacent = get_adjacent((x, y), grid_info);
                if adjacent.iter().all(|(_, num)| current < num) {
                    low_points.push(((x, y), current.to_owned()));
                }
            }
        }
    }
    low_points
}

fn part_one(low_points: &Vec<Location>) -> usize {
    low_points
        .into_iter()
        .fold(0, |acc, (_, num)| acc + *num as usize + 1)
}

fn get_adjacent_basin_recursively(coord: Coord, grid_info: &GridInfo) -> Vec<Location> {
    let current = grid_info.grid.get(&coord).unwrap();
    let adj = get_adjacent(coord, grid_info);

    let adjacent = adj
        .into_iter()
        .filter(|(_, value)| *value < 9 && value > current)
        .collect::<Vec<Location>>();

    let adjacent_rec = adjacent
        .iter()
        .flat_map(|(coord, _)| get_adjacent_basin_recursively(*coord, grid_info))
        .collect::<Vec<Location>>();

    [adjacent, adjacent_rec]
        .concat()
        .into_iter()
        .unique()
        .collect()
}

fn part_two(grid_info: &GridInfo, low_points: &Vec<Location>) -> u64 {
    let mut basins = low_points
        .iter()
        .map(|(coord, _)| get_adjacent_basin_recursively(*coord, grid_info).len() + 1)
        .collect::<Vec<usize>>();

    basins.sort_by(|a, b| b.cmp(a));

    basins
        .iter()
        .take(3)
        .fold(1, |acc, curr| acc * *curr as u64)
}

pub fn run() {
    let grid_info = parse_input();
    let low_points = get_low_points(&grid_info);
    let p1 = part_one(&low_points);
    println!("part 1 {}", p1);

    let p2 = part_two(&grid_info, &low_points);
    println!("part 2 {}", p2);
}
