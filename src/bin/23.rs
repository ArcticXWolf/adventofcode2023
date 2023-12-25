use std::fmt::{write, Display};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};
use itertools::Itertools;

enum Cell {
    Path,
    Slope(Point2Direction),
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Path),
            '^' => Ok(Self::Slope(Point2Direction::North)),
            '>' => Ok(Self::Slope(Point2Direction::East)),
            'v' => Ok(Self::Slope(Point2Direction::South)),
            '<' => Ok(Self::Slope(Point2Direction::West)),
            _ => Err(format!("Symbol is not a path -> {}", value)),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path => write!(f, "."),
            Self::Slope(Point2Direction::North) => write!(f, "^"),
            Self::Slope(Point2Direction::East) => write!(f, ">"),
            Self::Slope(Point2Direction::South) => write!(f, "v"),
            Self::Slope(Point2Direction::West) => write!(f, "<"),
            _ => unreachable!(),
        }
    }
}

struct PathGrid(PointGrid<isize, 2, Cell>);

impl From<&str> for PathGrid {
    fn from(value: &str) -> Self {
        let mut grid = PointGrid::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.char_indices() {
                match c {
                    '#' => continue,
                    _ => grid.insert(
                        Point2::new(x as isize, y as isize),
                        Cell::try_from(c).unwrap(),
                    ),
                }
            }
        }
        Self(grid)
    }
}

impl Display for PathGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
struct PathTree {
    nodes: Vec<Point2<isize>>,
    edges: Vec<(usize, usize, usize)>, // (from_node_idx, to_node_idx, weight)
    start_node_idx: usize,
    end_node_idx: usize,
}

impl From<&PathGrid> for PathTree {
    fn from(grid: &PathGrid) -> Self {
        let mut pt = PathTree::default();

        let start_node = Point2::new(1_isize, 0_isize);
        let mut queue = vec![(start_node, start_node, start_node, 0)];

        while let Some((current_position, last_position, last_node, current_edge_weight)) =
            queue.pop()
        {
            assert!(grid.0.get(&current_position).is_some());

            // todo slopes
            if let Some(Cell::Slope(direction)) = grid.0.get(&current_position) {
                let p = current_position.get_point_in_direction(direction, 1);
                queue.push((p, current_position, last_node, current_edge_weight + 1));
                continue;
            }

            let neighbors_pos = Point2Direction::all()
                .map(|d| current_position.get_point_in_direction(d, 1))
                .filter(|p| grid.0.get(p).is_some() && *p != last_position)
                .collect_vec();

            let (new_weight, new_last_node) = if neighbors_pos.len() >= 2 {
                pt.add(last_node, current_position, current_edge_weight);
                (1, current_position)
            } else {
                (current_edge_weight + 1, last_node)
            };

            if neighbors_pos.is_empty() {
                pt.add(last_node, current_position, current_edge_weight);
                pt.set_start_end(start_node, current_position);
            }

            for p in neighbors_pos {
                if let Some(Cell::Slope(direction)) = grid.0.get(&p) {
                    if Ok(*direction) == Point2Direction::try_from((current_position, p)) {
                        queue.push((p, current_position, new_last_node, new_weight));
                    }
                } else {
                    queue.push((p, current_position, new_last_node, new_weight));
                }
            }
        }

        pt
    }
}

impl PathTree {
    fn add(&mut self, source_node: Point2<isize>, destination_node: Point2<isize>, weight: usize) {
        let source_idx = if !self.nodes.contains(&source_node) {
            let idx = self.nodes.len();
            self.nodes.push(source_node);
            idx
        } else {
            self.nodes.iter().position(|&n| n == source_node).unwrap()
        };
        let destination_idx = if !self.nodes.contains(&destination_node) {
            let idx = self.nodes.len();
            self.nodes.push(destination_node);
            idx
        } else {
            self.nodes
                .iter()
                .position(|&n| n == destination_node)
                .unwrap()
        };

        if !self.edges.contains(&(source_idx, destination_idx, weight)) {
            self.edges.push((source_idx, destination_idx, weight));
        }
    }

    fn set_start_end(&mut self, start_node: Point2<isize>, end_node: Point2<isize>) {
        self.start_node_idx = if !self.nodes.contains(&start_node) {
            let idx = self.nodes.len();
            self.nodes.push(start_node);
            idx
        } else {
            self.nodes.iter().position(|&n| n == start_node).unwrap()
        };
        self.end_node_idx = if !self.nodes.contains(&end_node) {
            let idx = self.nodes.len();
            self.nodes.push(end_node);
            idx
        } else {
            self.nodes.iter().position(|&n| n == end_node).unwrap()
        };
    }

    fn highest_cost(&self, node_idx: usize) -> usize {
        if node_idx == self.end_node_idx {
            return 0;
        }

        let mut best_path = 0;
        for (src, dst, wgt) in &self.edges {
            if *src == node_idx {
                best_path = best_path.max(wgt + self.highest_cost(*dst));
            }
        }
        best_path
    }

    fn highest_cost_without_slopes(&self, path: &[usize]) -> usize {
        let node_idx = *path.last().unwrap();

        if node_idx == self.end_node_idx {
            return 0;
        }

        let mut best_path = 0;
        for (src, dst, wgt) in &self.edges {
            if *src == node_idx && !path.contains(dst) {
                best_path = best_path
                    .max(wgt + self.highest_cost_without_slopes(&[path, &[*dst]].concat()));
            }
            if *dst == node_idx && !path.contains(src) {
                best_path = best_path
                    .max(wgt + self.highest_cost_without_slopes(&[path, &[*src]].concat()));
            }
        }
        best_path
    }

    fn print_graphviz(&self) {
        println!("digraph AOC {{");
        for (src, dst, wgt) in &self.edges {
            println!("{} -> {} [label={}]", src, dst, wgt);
        }
        println!("}}");
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let pathgrid = PathGrid::from(_input);
    let pathtree = PathTree::from(&pathgrid);
    Some(pathtree.highest_cost(pathtree.start_node_idx))
}

pub fn part_two(_input: &str) -> Option<usize> {
    let pathgrid = PathGrid::from(_input);
    let pathtree = PathTree::from(&pathgrid);
    Some(pathtree.highest_cost_without_slopes(&vec![pathtree.start_node_idx]))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(94));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(154));
    }
}
