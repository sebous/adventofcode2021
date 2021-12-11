use std::{collections::HashMap, ops::Range};

use itertools::{Itertools, Product};

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
        let a = match *x != 0 && *y != 0 && diagonals {
            true => Some((x - 1, y - 1)),
            false => None,
        };
        let b = match *y != 0 {
            true => Some((*x, y - 1)),
            false => None,
        };
        let c = match *x + 1 < self.width && *y != 0 && diagonals {
            true => Some((x + 1, y - 1)),
            false => None,
        };
        let d = match *x + 1 < self.width {
            true => Some((x + 1, *y)),
            false => None,
        };
        let e = match *x + 1 < self.width && *y + 1 < self.height && diagonals {
            true => Some((x + 1, y + 1)),
            false => None,
        };
        let f = match *y + 1 < self.height {
            true => Some((*x, y + 1)),
            false => None,
        };
        let g = match *x != 0 && *y + 1 < self.height && diagonals {
            true => Some((x - 1, y + 1)),
            false => None,
        };
        let h = match *x != 0 {
            true => Some((x - 1, *y)),
            false => None,
        };
        [a, b, c, d, e, f, g, h].into_iter()
    }

    pub fn iter_coords(&self) -> Product<Range<usize>, Range<usize>> {
        (0..self.width).cartesian_product(0..self.height)
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn get_adjacent() {
        let grid = grid_test_init();

        assert_eq!(
            grid.get_adjacent_coords(&(1, 1), false)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(1, 0), (2, 1), (1, 2), (0, 1)]
        );

        assert_eq!(
            grid.get_adjacent_coords(&(0, 1), false)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(0, 0), (1, 1), (0, 2)]
        );

        assert_eq!(
            grid.get_adjacent_coords(&(2, 2), false)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(2, 1), (1, 2)]
        );

        assert_eq!(
            grid.get_adjacent_coords(&(1, 1), true)
                .filter_map(|c| c)
                .collect_vec(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
                (1, 2),
                (0, 2),
                (0, 1)
            ]
        );

        assert_eq!(
            grid.get_adjacent_coords(&(0, 0), true)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(1, 0), (1, 1), (0, 1)]
        );

        assert_eq!(
            grid.get_adjacent_coords(&(2, 1), true)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(1, 0), (2, 0), (2, 2), (1, 2), (1, 1)]
        );
        assert_eq!(
            grid.get_adjacent_coords(&(2, 2), true)
                .filter_map(|c| c)
                .collect_vec(),
            vec![(1, 1), (2, 1), (1, 2)]
        );
    }
}