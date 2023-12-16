use std::{collections::HashSet, fmt::Display};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};
use itertools::Itertools;

enum Cell {
    MirrorForwardSlash,
    MirrorBackwardSlash,
    SplitterHorizontal,
    SplitterVertical,
}

impl Cell {
    fn adjust_direction(&self, direction: &Point2Direction) -> Vec<Point2Direction> {
        match (self, direction) {
            (Cell::MirrorForwardSlash, Point2Direction::North) => vec![Point2Direction::East],
            (Cell::MirrorForwardSlash, Point2Direction::East) => vec![Point2Direction::North],
            (Cell::MirrorForwardSlash, Point2Direction::South) => vec![Point2Direction::West],
            (Cell::MirrorForwardSlash, Point2Direction::West) => vec![Point2Direction::South],
            (Cell::MirrorBackwardSlash, Point2Direction::North) => vec![Point2Direction::West],
            (Cell::MirrorBackwardSlash, Point2Direction::East) => vec![Point2Direction::South],
            (Cell::MirrorBackwardSlash, Point2Direction::South) => vec![Point2Direction::East],
            (Cell::MirrorBackwardSlash, Point2Direction::West) => vec![Point2Direction::North],
            (Cell::SplitterHorizontal, Point2Direction::North) => {
                vec![Point2Direction::East, Point2Direction::West]
            }
            (Cell::SplitterHorizontal, Point2Direction::South) => {
                vec![Point2Direction::East, Point2Direction::West]
            }
            (Cell::SplitterVertical, Point2Direction::East) => {
                vec![Point2Direction::North, Point2Direction::South]
            }
            (Cell::SplitterVertical, Point2Direction::West) => {
                vec![Point2Direction::North, Point2Direction::South]
            }
            (_, d) => vec![*d],
        }
    }
}

impl TryFrom<char> for Cell {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '/' => Ok(Self::MirrorForwardSlash),
            '\\' => Ok(Self::MirrorBackwardSlash),
            '-' => Ok(Self::SplitterHorizontal),
            '|' => Ok(Self::SplitterVertical),
            _ => Err("Could not match symbol".to_string()),
        }
    }
    type Error = String;
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MirrorForwardSlash => write!(f, "/"),
            Self::MirrorBackwardSlash => write!(f, "\\"),
            Self::SplitterHorizontal => write!(f, "-"),
            Self::SplitterVertical => write!(f, "|"),
        }
    }
}

struct Cave {
    grid: PointGrid<isize, 2, Cell>,
    travelled_path: HashSet<(Point2<isize>, Point2Direction)>,
    energized_cells: HashSet<Point2<isize>>,
}

impl Cave {
    fn reset(&mut self) {
        self.energized_cells = HashSet::default();
        self.travelled_path = HashSet::default();
    }

    fn trace(&mut self, starting_position: &Point2<isize>, starting_direction: &Point2Direction) {
        let (min, max) = self.grid.dimensions();

        let mut open_nodes: Vec<(Point2<isize>, Point2Direction)> =
            vec![(*starting_position, *starting_direction)];
        while let Some((current_pos, current_direction)) = open_nodes.pop() {
            self.energized_cells.insert(current_pos);
            self.travelled_path.insert((current_pos, current_direction));

            let mut new_directions = vec![current_direction];
            if let Some(cell) = self.grid.get(&current_pos) {
                new_directions = cell.adjust_direction(&current_direction);
            }

            for d in &new_directions {
                let new_pos = current_pos.get_point_in_direction(d, 1);
                if new_pos.0[0] < min.0[0]
                    || new_pos.0[1] < min.0[1]
                    || new_pos.0[0] > max.0[0]
                    || new_pos.0[1] > max.0[1]
                {
                    continue;
                }

                if !self.travelled_path.contains(&(new_pos, *d)) {
                    open_nodes.push((new_pos, *d));
                }
            }
        }
    }
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        let mut grid = PointGrid::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.char_indices() {
                if let Ok(cell) = Cell::try_from(c) {
                    grid.insert(Point2::new(x as isize, y as isize), cell);
                }
            }
        }
        let travelled_path = HashSet::default();
        let energized_cells = HashSet::default();

        Self {
            grid,
            travelled_path,
            energized_cells,
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.grid.dimensions();
        writeln!(f, "Grid ({}, {}):", min, max)?;
        for y in min.0[1]..(max.0[1] + 1) {
            for x in min.0[0]..(max.0[0] + 1) {
                let pos = &Point2::new(x, y);
                if let Some(u) = self.grid.get(pos) {
                    write!(f, "{}", u)?;
                } else {
                    let pathnodes = self
                        .travelled_path
                        .iter()
                        .filter(|(p, _)| p == pos)
                        .collect_vec();
                    if pathnodes.is_empty() {
                        write!(f, " ")?;
                    } else if pathnodes.len() == 1 {
                        write!(f, "{}", pathnodes.first().unwrap().1)?;
                    } else if pathnodes.len() < 10 {
                        write!(f, "{}", pathnodes.len())?;
                    } else {
                        write!(f, "*")?;
                    }
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut cave = Cave::from(_input);
    cave.trace(&Point2::new(0, 0), &Point2Direction::East);
    Some(cave.energized_cells.len())
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut cave = Cave::from(_input);

    let (min, max) = cave.grid.dimensions();

    let mut best_cells = HashSet::default();
    let mut best_path = HashSet::default();
    for x in min.0[0]..(max.0[0] + 1) {
        cave.trace(&Point2::new(x, min.0[1]), &Point2Direction::South);
        if best_cells.len() < cave.energized_cells.len() {
            best_cells = cave.energized_cells.clone();
            best_path = cave.travelled_path.clone();
        }
        cave.reset();

        cave.trace(&Point2::new(x, max.0[1]), &Point2Direction::North);
        if best_cells.len() < cave.energized_cells.len() {
            best_cells = cave.energized_cells.clone();
            best_path = cave.travelled_path.clone();
        }
        cave.reset();
    }
    for y in min.0[1]..(max.0[1] + 1) {
        cave.trace(&Point2::new(min.0[0], y), &Point2Direction::East);
        if best_cells.len() < cave.energized_cells.len() {
            best_cells = cave.energized_cells.clone();
            best_path = cave.travelled_path.clone();
        }
        cave.reset();

        cave.trace(&Point2::new(max.0[0], y), &Point2Direction::West);
        if best_cells.len() < cave.energized_cells.len() {
            best_cells = cave.energized_cells.clone();
            best_path = cave.travelled_path.clone();
        }
        cave.reset();
    }

    cave.travelled_path = best_path;
    cave.energized_cells = best_cells;
    Some(cave.energized_cells.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(46));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(51));
    }
}
