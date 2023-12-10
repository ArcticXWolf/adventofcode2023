use std::{collections::HashSet, fmt::Display};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};
use itertools::Itertools;

enum PipeShape {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    EastSouth,
}

impl PipeShape {
    fn get_exit_directions(&self) -> [Point2Direction; 2] {
        match self {
            Self::NorthSouth => [Point2Direction::North, Point2Direction::South],
            Self::EastWest => [Point2Direction::East, Point2Direction::West],
            Self::NorthEast => [Point2Direction::North, Point2Direction::East],
            Self::NorthWest => [Point2Direction::North, Point2Direction::West],
            Self::SouthWest => [Point2Direction::South, Point2Direction::West],
            Self::EastSouth => [Point2Direction::East, Point2Direction::South],
        }
    }

    fn get_other_exit_direction(&self, direction: &Point2Direction) -> Option<Point2Direction> {
        return self
            .get_exit_directions()
            .iter()
            .find(|&d| *d != *direction)
            .copied();
    }

    fn from_directions(d1: &Point2Direction, d2: &Point2Direction) -> Self {
        let dt = if d1 < d2 { (d1, d2) } else { (d2, d1) };

        match dt {
            (Point2Direction::North, Point2Direction::South) => Self::NorthSouth,
            (Point2Direction::East, Point2Direction::West) => Self::EastWest,
            (Point2Direction::North, Point2Direction::East) => Self::NorthEast,
            (Point2Direction::North, Point2Direction::West) => Self::NorthWest,
            (Point2Direction::South, Point2Direction::West) => Self::SouthWest,
            (Point2Direction::East, Point2Direction::South) => Self::EastSouth,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for PipeShape {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::EastSouth),
            _ => Err(format!("Unknown pipeshape {}", value)),
        }
    }
}

impl Display for PipeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NorthSouth => "|",
                Self::EastWest => "-",
                Self::NorthEast => "L",
                Self::NorthWest => "J",
                Self::SouthWest => "7",
                Self::EastSouth => "F",
            }
        )
    }
}

fn parse_input(s: &str) -> (PointGrid<isize, 2, PipeShape>, Point2<isize>) {
    let mut starting_pos = Point2::new(0, 0);
    let mut grid = PointGrid::default();

    for (line_idx, line) in s.trim().lines().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c == 'S' {
                starting_pos = Point2::new(char_idx as isize, line_idx as isize);
                continue;
            } else if c == '.' {
                continue;
            }

            grid.insert(
                Point2::new(char_idx as isize, line_idx as isize),
                c.try_into().unwrap(),
            );
        }
    }

    grid.insert(
        starting_pos,
        identify_tile_by_surroundings(&starting_pos, &grid),
    );

    (grid, starting_pos)
}

fn identify_tile_by_surroundings(
    position: &Point2<isize>,
    grid: &PointGrid<isize, 2, PipeShape>,
) -> PipeShape {
    let mut found_directions = vec![];
    for d in Point2Direction::all() {
        let neighbor = position.get_point_in_direction(d, 1);
        if let Some(neighbor_pipe_shape) = grid.get(&neighbor) {
            if neighbor_pipe_shape
                .get_exit_directions()
                .contains(&d.direction_left().direction_left())
            {
                found_directions.push(d);
            }
        }
        if found_directions.len() == 2 {
            return PipeShape::from_directions(
                found_directions.iter().nth(0).unwrap(),
                found_directions.iter().nth(1).unwrap(),
            );
        }
    }
    unreachable!()
}

fn trace_loop(
    grid: &PointGrid<isize, 2, PipeShape>,
    starting_pos: Point2<isize>,
) -> Vec<Point2<isize>> {
    let mut node_path = vec![];
    let mut current_node = starting_pos;
    let mut next_direction = grid
        .get(&starting_pos)
        .unwrap()
        .get_exit_directions()
        .first()
        .unwrap()
        .clone();

    while current_node.get_point_in_direction(&next_direction, 1) != starting_pos {
        node_path.push(current_node);
        current_node = current_node.get_point_in_direction(&next_direction, 1);
        if let Some(ps) = grid.get(&current_node) {
            if let Some(new_direction) =
                ps.get_other_exit_direction(&next_direction.direction_left().direction_left())
            {
                next_direction = new_direction;
                continue;
            }
        }

        // this path is no loop!
        unreachable!()
    }
    node_path.push(current_node);

    node_path
}

fn fill_loop(
    grid: &PointGrid<isize, 2, PipeShape>,
    loop_path: &[Point2<isize>],
) -> Option<Vec<Point2<isize>>> {
    // try lefthanded, then righthanded
    let left_handed_try = fill_loop_onehanded(grid, loop_path, true);
    if left_handed_try.is_some() {
        return left_handed_try;
    }
    fill_loop_onehanded(grid, loop_path, false)
}

fn fill_loop_onehanded(
    grid: &PointGrid<isize, 2, PipeShape>,
    loop_path: &[Point2<isize>],
    lefthanded: bool,
) -> Option<Vec<Point2<isize>>> {
    let mut inside_nodes = HashSet::new();

    let starting_loop_node = *loop_path.first().unwrap();
    let mut current_loop_node = starting_loop_node;
    let mut current_direction = *Point2Direction::all()
        .filter(|d| {
            starting_loop_node.get_point_in_direction(d, 1) == *loop_path.iter().nth(1).unwrap()
        })
        .next()
        .unwrap();

    // loop over the whole nodeloop
    while current_loop_node.get_point_in_direction(&current_direction, 1) != starting_loop_node {
        // add nodes to inside_nodes in a direction perpendicular to our moving direction
        let fill_direction = if lefthanded {
            current_direction.direction_left()
        } else {
            current_direction.direction_right()
        };
        if !fill_towards_direction(
            &grid,
            loop_path,
            current_loop_node,
            &fill_direction,
            &mut inside_nodes,
        ) {
            // we are filling the outside, abort..
            return None;
        }

        // follow the loop
        current_loop_node = current_loop_node.get_point_in_direction(&current_direction, 1);

        // add nodes to inside_nodes in a direction perpendicular to our moving direction
        let fill_direction = if lefthanded {
            current_direction.direction_left()
        } else {
            current_direction.direction_right()
        };
        if !fill_towards_direction(
            &grid,
            loop_path,
            current_loop_node,
            &fill_direction,
            &mut inside_nodes,
        ) {
            // we are filling the outside, abort..
            return None;
        }

        let pipe_shape = grid.get(&current_loop_node).unwrap();
        current_direction = pipe_shape
            .get_other_exit_direction(&current_direction.direction_left().direction_left())
            .unwrap();
    }

    Some(inside_nodes.into_iter().collect_vec())
}

fn fill_towards_direction(
    grid: &PointGrid<isize, 2, PipeShape>,
    loop_path: &[Point2<isize>],
    starting_pos: Point2<isize>,
    direction: &Point2Direction,
    inside_nodes: &mut HashSet<Point2<isize>>,
) -> bool {
    let mut current_node = starting_pos;
    while !loop_path.contains(&current_node.get_point_in_direction(direction, 1)) {
        inside_nodes.insert(current_node.get_point_in_direction(direction, 1));
        current_node = current_node.get_point_in_direction(direction, 1);

        // if out of bounds, then we are filling the outside, abort..
        let dim = grid.dimensions();
        if current_node.0[0] < dim.0 .0[0]
            || current_node.0[1] < dim.0 .0[1]
            || dim.1 .0[0] < current_node.0[0]
            || dim.1 .0[1] < current_node.0[1]
        {
            return false;
        }
    }
    true
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (grid, starting_pos) = parse_input(_input);
    let loop_path = trace_loop(&grid, starting_pos);
    Some((loop_path.len() / 2) as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (grid, starting_pos) = parse_input(_input);
    let loop_path = trace_loop(&grid, starting_pos);
    let inside_nodes = fill_loop(&grid, &loop_path);

    if let Some(nodes) = inside_nodes {
        Some(nodes.len() as u32)
    } else {
        None
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(80));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(10));
    }
}
