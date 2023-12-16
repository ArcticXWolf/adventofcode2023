use std::fmt::Display;

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Round,
    Cube,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Cube => write!(f, "#"),
            Cell::Round => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Platform(PointGrid<isize, 2, Cell>);

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        let mut platform = Platform::default();
        for (y, row) in value.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '#' => platform
                        .0
                        .insert(Point2::new(x as isize, y as isize), Cell::Cube),
                    'O' => platform
                        .0
                        .insert(Point2::new(x as isize, y as isize), Cell::Round),
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }
        platform
    }
}

impl Platform {
    fn tilt(&mut self, direction: Point2Direction) {
        let mut new_grid = PointGrid::default();

        let (dim_min, dim_max) = self.0.dimensions();
        let bounds = match direction {
            Point2Direction::North => (
                (0..dim_max.0[0] + 1).collect_vec(),
                (0..dim_max.0[1] + 1).collect_vec(),
                dim_min.0[1],
                1,
            ),
            Point2Direction::South => (
                (0..dim_max.0[0] + 1).collect_vec(),
                (0..dim_max.0[1] + 1).rev().collect_vec(),
                dim_max.0[1],
                -1,
            ),
            Point2Direction::West => (
                (0..dim_max.0[1] + 1).collect_vec(),
                (0..dim_max.0[0] + 1).collect_vec(),
                dim_min.0[0],
                1,
            ),
            Point2Direction::East => (
                (0..dim_max.0[1] + 1).collect_vec(),
                (0..dim_max.0[0] + 1).rev().collect_vec(),
                dim_max.0[0],
                -1,
            ),
            _ => unreachable!(),
        };
        for i in bounds.0.clone() {
            let mut next_push_position = bounds.2;

            for j in bounds.1.clone() {
                let (current_p, next_p) = match direction {
                    Point2Direction::North => {
                        (Point2::new(i, j), Point2::new(i, next_push_position))
                    }
                    Point2Direction::South => {
                        (Point2::new(i, j), Point2::new(i, next_push_position))
                    }
                    Point2Direction::East => {
                        (Point2::new(j, i), Point2::new(next_push_position, i))
                    }
                    Point2Direction::West => {
                        (Point2::new(j, i), Point2::new(next_push_position, i))
                    }
                    _ => unreachable!(),
                };
                if let Some(c) = self.0.get(&current_p) {
                    match c {
                        Cell::Cube => {
                            new_grid.insert(current_p, Cell::Cube);
                            next_push_position = j + bounds.3;
                        }
                        Cell::Round => {
                            new_grid.insert(next_p, Cell::Round);
                            next_push_position += bounds.3;
                        }
                    }
                }
            }
        }

        self.0 = new_grid;
    }

    fn cycle(&mut self) {
        self.tilt(Point2Direction::North);
        self.tilt(Point2Direction::West);
        self.tilt(Point2Direction::South);
        self.tilt(Point2Direction::East);
    }

    fn calculate_load(&self) -> isize {
        let mut load = 0;
        let (_, dim_max) = self.0.dimensions();
        for p in self.0.iter_full_bounds() {
            if let Some(Cell::Round) = self.0.get(&p) {
                load += dim_max.0[1] - p.0[1] + 1;
            }
        }
        load
    }
}

pub fn part_one(_input: &str) -> Option<isize> {
    let mut platform = Platform::from(_input.trim());
    platform.tilt(Point2Direction::North);
    Some(platform.calculate_load())
}

pub fn part_two(_input: &str) -> Option<isize> {
    let mut platform = Platform::from(_input.trim());
    let mut cache = vec![platform.0.clone()];

    let mut i = 0;
    let mut cycle_found = false;
    let limit = 1000000000;
    while i < limit {
        platform.cycle();

        if !cycle_found {
            if cache.contains(&platform.0) {
                let cycle_start = cache.iter().position(|p| p == &platform.0).unwrap();
                let cycle_length = i - cycle_start + 1;
                cycle_found = true;

                // fast forward
                i += ((limit - i - cycle_length) / cycle_length) * cycle_length;
            } else {
                cache.push(platform.0.clone());
            }
        }
        i += 1;
    }
    Some(platform.calculate_load())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(136));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(64));
    }
}
