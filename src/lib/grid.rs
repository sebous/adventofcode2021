use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use itertools::Itertools;

pub type Coord = (usize, usize);
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub map: HashMap<Coord, T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    /// returns iterator over adjacent coords
    pub fn get_adjacent_coords(
        &self,
        coord: &Coord,
        diagonals: bool,
    ) -> impl Iterator<Item = Option<Coord>> {
        let (x, y) = coord;
        let (min_x, min_y) = self.get_min_coord();
        let left_top = match *x != 0 && *y != 0 && diagonals {
            true => Some((x - 1, y - 1)),
            false => None,
        };
        let top = match *y != 0 {
            true => Some((*x, y - 1)),
            false => None,
        };
        let right_top = match *x + 1 < (self.width + min_x) && *y != 0 && diagonals {
            true => Some((x + 1, y - 1)),
            false => None,
        };
        let right = match *x + 1 < (self.width + min_x) {
            true => Some((x + 1, *y)),
            false => None,
        };
        let right_bot =
            match *x + 1 < (self.width + min_x) && *y + 1 < (self.height + min_y) && diagonals {
                true => Some((x + 1, y + 1)),
                false => None,
            };
        let bot = match *y + 1 < (self.height + min_y) {
            true => Some((*x, y + 1)),
            false => None,
        };
        let left_bot = match *x != 0 && *y + 1 < (self.height + min_y) && diagonals {
            true => Some((x - 1, y + 1)),
            false => None,
        };
        let left = match *x != 0 {
            true => Some((x - 1, *y)),
            false => None,
        };
        let existing = [
            left_top, top, right_top, left, right, left_bot, bot, right_bot,
        ];
        existing.into_iter()

        // .filter_map(|item| *item)
        // .collect_vec();

        // existing.into_iter()
    }

    pub fn get_min_coord(&self) -> Coord {
        let min_x = self
            .map
            .keys()
            .into_iter()
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .0;
        let min_y = self
            .map
            .keys()
            .into_iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;
        (min_x, min_y)
    }

    fn get_max_coord(&self) -> Coord {
        let max_x = self
            .map
            .keys()
            .into_iter()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .0;
        let max_y = self
            .map
            .keys()
            .into_iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;
        (max_x, max_y)
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        let (min_x, min_y) = self.get_min_coord();
        (min_x..self.width + min_x)
            .cartesian_product(min_y..self.height + min_y)
            .sorted()
    }
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let (min_x, min_y) = self.get_min_coord();
        for y in min_y..self.height + min_y {
            for x in min_x..self.width + min_x {
                let val = self.map.get(&(x, y)).unwrap();
                result.push_str(format!("{}", val).as_str());
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod grid_tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use super::{Coord, Grid};

    /// initializes 3x3 grid for testing
    fn grid_test_init() -> Grid<usize> {
        let width = 3;
        let height = 3;
        let map = (0..width)
            .cartesian_product(0..height)
            .map(|coord| (coord, 0))
            .collect::<HashMap<Coord, usize>>();
        Grid { map, width, height }
    }

    #[test]
    fn iter() {
        let grid = grid_test_init();
        assert_eq!(grid.iter_coords().next().unwrap(), (0, 0));
        assert_eq!(grid.iter_coords().last().unwrap(), (2, 2));
    }

    // #[test]
    // fn get_adjacent() {
    //     let grid = grid_test_init();

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(1, 1), false)
    //             .filter_map(|x| x)
    //             .sorted()
    //             .collect_vec(),
    //         vec![(1, 0), (2, 1), (1, 2), (0, 1)]
    //     );

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(0, 1), false)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![(0, 0), (1, 1), (0, 2)]
    //     );

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(2, 2), false)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![(2, 1), (1, 2)]
    //     );

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(1, 1), true)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![
    //             (0, 0),
    //             (1, 0),
    //             (2, 0),
    //             (2, 1),
    //             (2, 2),
    //             (1, 2),
    //             (0, 2),
    //             (0, 1)
    //         ]
    //     );

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(0, 0), true)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![(1, 0), (1, 1), (0, 1)]
    //     );

    //     assert_eq!(
    //         grid.get_adjacent_coords(&(2, 1), true)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![(1, 0), (2, 0), (2, 2), (1, 2), (1, 1)]
    //     );
    //     assert_eq!(
    //         grid.get_adjacent_coords(&(2, 2), true)
    //             .filter_map(|x| x)
    //             .collect_vec(),
    //         vec![(1, 1), (2, 1), (1, 2)]
    //     );
    // }
}
