use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

// For part2 my initial solution was to run along the loop and then mark all
// nodes on the left (in the direction of running) as inside. If we would go out
// of bounds during this, we know that we had to mark all nodes on the right
// instead of left. (This is the same algorithm as the maze solving algorithm)
// You can see that solution in my previous commit.
//
// Now, I am using a simple scanline algoritm and flip inside/outside if I
// encounter a pipe on the loop.

#[derive(Debug, Clone, Copy)]
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
                found_directions.get(0).unwrap(),
                found_directions.get(1).unwrap(),
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
    let mut next_direction = *grid
        .get(&starting_pos)
        .unwrap()
        .get_exit_directions()
        .first()
        .unwrap();

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

fn find_loop_fill(
    grid: &PointGrid<isize, 2, PipeShape>,
    loop_path: &[Point2<isize>],
) -> Vec<Point2<isize>> {
    let mut inside_nodes = vec![];
    let dim = grid.dimensions();
    for y in dim.0 .0[1]..(dim.1 .0[1] + 1) {
        let mut inside = false;
        let mut last_corner = None;

        for x in dim.0 .0[0]..(dim.1 .0[0] + 1) {
            if loop_path.contains(&Point2::new(x, y)) {
                let pipe_shape = grid.get(&Point2::new(x, y));
                match (pipe_shape, last_corner) {
                    (None, _) => {}
                    (Some(PipeShape::NorthSouth), _)
                    | (Some(PipeShape::SouthWest), Some(PipeShape::NorthEast))
                    | (Some(PipeShape::NorthWest), Some(PipeShape::EastSouth)) => {
                        inside = !inside;
                        last_corner = None;
                        continue;
                    }
                    (Some(PipeShape::EastWest), _)
                    | (Some(PipeShape::NorthWest), Some(PipeShape::NorthEast))
                    | (Some(PipeShape::SouthWest), Some(PipeShape::EastSouth)) => continue,
                    (Some(PipeShape::NorthEast), _) | (Some(PipeShape::EastSouth), _) => {
                        last_corner = pipe_shape.copied();
                        continue;
                    }
                    _ => {
                        println!(
                            "P{:?} I{:?} PS{:?} LC{:?}",
                            Point2::new(x, y),
                            inside,
                            pipe_shape,
                            last_corner
                        );
                        unreachable!()
                    }
                }
            }

            if inside {
                inside_nodes.push(Point2::new(x, y));
            }
        }
    }

    inside_nodes
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (grid, starting_pos) = parse_input(_input);
    let loop_path = trace_loop(&grid, starting_pos);
    Some((loop_path.len() / 2) as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (grid, starting_pos) = parse_input(_input);
    let loop_path = trace_loop(&grid, starting_pos);
    let inside_nodes = find_loop_fill(&grid, &loop_path);

    Some(inside_nodes.len() as u32)
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
