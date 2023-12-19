use advent_of_code::algebra_helpers::{Point2, Point2Direction};
use itertools::Itertools;

struct Instruction {
    direction: Point2Direction,
    distance: isize,
    color_direction: Point2Direction,
    color_distance: isize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let split = value.splitn(3, ' ').collect_vec();
        let direction = match split[0] {
            "U" => Point2Direction::North,
            "R" => Point2Direction::East,
            "D" => Point2Direction::South,
            "L" => Point2Direction::West,
            _ => unreachable!(),
        };
        let distance = split[1].parse::<isize>().unwrap();
        let color_direction = match split[2].chars().nth(7) {
            Some('0') => Point2Direction::East,
            Some('1') => Point2Direction::South,
            Some('2') => Point2Direction::West,
            Some('3') => Point2Direction::North,
            _ => unimplemented!(),
        };
        let color_distance = isize::from_str_radix(&split[2][2..7], 16).unwrap();
        Self {
            direction,
            distance,
            color_direction,
            color_distance,
        }
    }
}

fn calculate_vertices(instructions: &[Instruction], use_colors: bool) -> Vec<Point2<isize>> {
    let mut current_vertex = Point2::new(0_isize, 0_isize);
    let mut vertices = vec![current_vertex];

    for i in instructions {
        let (direction, distance) = if use_colors {
            (i.color_direction, i.color_distance)
        } else {
            (i.direction, i.distance)
        };

        current_vertex = current_vertex.get_point_in_direction(&direction, distance);
        vertices.push(current_vertex);
    }
    vertices
}

fn shoelace_theorem(vertices: &[Point2<isize>]) -> isize {
    let mut area = 0;
    for i in 0..(vertices.len() - 1) {
        let vertex1 = vertices.get(i).unwrap();
        let vertex2 = vertices.get(i + 1).unwrap();
        let line_x = (vertex1.0[0] - vertex2.0[0]).abs();
        let line_y = (vertex1.0[1] - vertex2.0[1]).abs();
        let column = vertex1.0[0] * vertex2.0[1] - vertex1.0[1] * vertex2.0[0];
        area += column + line_x + line_y;
    }

    area / 2 + 1
}

pub fn part_one(_input: &str) -> Option<isize> {
    let instructions: Vec<_> = _input.trim().lines().map(Instruction::from).collect();

    let vertices = calculate_vertices(&instructions, false);
    Some(shoelace_theorem(&vertices))
}

pub fn part_two(_input: &str) -> Option<isize> {
    let instructions: Vec<_> = _input.trim().lines().map(Instruction::from).collect();

    let vertices = calculate_vertices(&instructions, true);
    Some(shoelace_theorem(&vertices))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(62));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(952408144115));
    }
}
