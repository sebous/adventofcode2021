use std::collections::{BinaryHeap, HashMap};

use crate::lib::{
    grid::{Coord, Grid},
    input::load_input,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Edge {
    coord: Coord,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(multiplier: usize) -> Grid<usize> {
    let input = load_input("15");
    let mut map = HashMap::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    for y_multi in 0..multiplier {
        for (y, line) in input.lines().enumerate() {
            for x_multi in 0..multiplier {
                for (x, val) in line.chars().enumerate() {
                    let val = val.to_digit(10).unwrap() as usize;
                    let val = (val + x_multi + y_multi - 1) % 9 + 1;
                    map.insert((x + x_multi * width, y + y_multi * height), val);
                }
            }
        }
    }
    Grid {
        map,
        height: height * multiplier,
        width: width * multiplier,
    }
}

fn shortest_path(grid: &Grid<usize>, target: &Coord) -> usize {
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    let start: Coord = (0, 0);
    heap.push(Edge {
        coord: start,
        cost: 0,
    });

    loop {
        if let Some(edge) = heap.pop() {
            if edge.coord == *target {
                return edge.cost;
            }

            if edge.cost <= *costs.get(&edge.coord).unwrap_or(&usize::MAX) {
                for adjacent in grid
                    .get_adjacent_coords(&edge.coord, false)
                    .filter_map(|x| x)
                {
                    let next_edge = Edge {
                        cost: edge.cost + grid.map.get(&adjacent).unwrap(),
                        coord: adjacent,
                    };
                    if next_edge.cost < *costs.get(&adjacent).unwrap_or(&usize::MAX) {
                        heap.push(next_edge);
                        costs.insert(adjacent, next_edge.cost);
                    }
                }
            }
        }
    }
}

pub fn run() {
    let grid = parse(1);
    let p1 = shortest_path(&grid, &(grid.width - 1, grid.height - 1));
    println!("part 1: {}", p1);

    let grid2 = parse(5);
    let p2 = shortest_path(&grid2, &(grid2.width - 1, grid2.height - 1));
    println!("part 2: {}", p2);
}
