use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use advent_of_code::algebra_helpers::{Point2, Point2Direction, PointGrid};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point2<isize>,
    direction_counter: u32,
    entered_from: Point2Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // we flip the Ord here, so the max-heap becomes a min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.direction_counter.cmp(&self.direction_counter))
            .then_with(|| self.position.0[0].cmp(&other.position.0[0]))
            .then_with(|| self.position.0[1].cmp(&other.position.0[1]))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct City {
    grid: PointGrid<isize, 2, u32>,
}

impl City {
    fn cubicle_dijkstra(
        &self,
        starting_position: Point2<isize>,
        ending_position: Point2<isize>,
    ) -> Option<(u32, HashMap<Point2<isize>, Point2Direction>)> {
        let mut closed_nodes: HashMap<(Point2<isize>, Point2Direction, u32), u32> =
            HashMap::default();
        let mut path: HashMap<Point2<isize>, Point2Direction> = HashMap::default();
        let mut open_nodes = BinaryHeap::new();

        open_nodes.push(State {
            cost: 0,
            position: starting_position,
            direction_counter: 0,
            entered_from: Point2Direction::North,
        });

        while let Some(State {
            cost,
            position,
            direction_counter,
            entered_from,
        }) = open_nodes.pop()
        {
            if closed_nodes.contains_key(&(position, entered_from, direction_counter)) {
                continue;
            }

            closed_nodes.insert((position, entered_from, direction_counter), cost);
            path.insert(position, entered_from);

            if position == ending_position {
                break;
            }

            for d in Point2Direction::all() {
                // dont go backwards
                if *d == entered_from {
                    continue;
                }

                // keep track of the length of the straight line
                let new_dir_count = if entered_from == d.direction_flip() {
                    direction_counter + 1
                } else {
                    0
                };

                // do not proceed on straight lines longer than 3
                if new_dir_count > 2 {
                    continue;
                }

                let new_pos = position.get_point_in_direction(d, 1);

                if let Some(cell_cost) = self.grid.get(&new_pos) {
                    open_nodes.push(State {
                        cost: cost + cell_cost,
                        position: new_pos,
                        direction_counter: new_dir_count,
                        entered_from: d.direction_flip(),
                    });
                }
            }
        }

        if let Some(result) = closed_nodes
            .iter()
            .filter_map(|(&(p, _, _), c)| if p == ending_position { Some(*c) } else { None })
            .min()
        {
            Some((result, path))
        } else {
            None
        }
    }

    fn ultra_cubicle_dijkstra(
        &self,
        starting_position: Point2<isize>,
        ending_position: Point2<isize>,
    ) -> Option<(u32, HashMap<Point2<isize>, Point2Direction>)> {
        let mut closed_nodes: HashMap<(Point2<isize>, Point2Direction, u32), u32> =
            HashMap::default();
        let mut path: HashMap<Point2<isize>, Point2Direction> = HashMap::default();
        let mut open_nodes = BinaryHeap::new();

        for (d, p) in Point2Direction::all()
            .map(|d| (d, self.get_cost_between_points(starting_position, d, 4)))
        {
            if let Some((c, pos)) = p {
                open_nodes.push(State {
                    cost: c,
                    position: pos,
                    direction_counter: 4,
                    entered_from: d.direction_flip(),
                });
            }
        }

        while let Some(State {
            cost,
            position,
            direction_counter,
            entered_from,
        }) = open_nodes.pop()
        {
            if closed_nodes.contains_key(&(position, entered_from, direction_counter)) {
                continue;
            }

            closed_nodes.insert((position, entered_from, direction_counter), cost);
            path.insert(position, entered_from);

            if position == ending_position {
                break;
            }

            for d in Point2Direction::all() {
                // dont go backwards
                if *d == entered_from {
                    continue;
                }

                // keep track of the length of the straight line
                let (new_dir_count, pos_cost) = if entered_from == d.direction_flip() {
                    (
                        direction_counter + 1,
                        self.get_cost_between_points(position, d, 1),
                    )
                } else {
                    (4, self.get_cost_between_points(position, d, 4))
                };

                // do not proceed on straight lines longer than 3
                if new_dir_count > 10 {
                    continue;
                }

                if let Some((cell_cost, new_pos)) = pos_cost {
                    open_nodes.push(State {
                        cost: cost + cell_cost,
                        position: new_pos,
                        direction_counter: new_dir_count,
                        entered_from: d.direction_flip(),
                    });
                }
            }
        }

        if let Some(result) = closed_nodes
            .iter()
            .filter_map(|(&(p, _, _), c)| if p == ending_position { Some(*c) } else { None })
            .min()
        {
            Some((result, path))
        } else {
            None
        }
    }

    fn get_cost_between_points(
        &self,
        start: Point2<isize>,
        direction: &Point2Direction,
        distance: isize,
    ) -> Option<(u32, Point2<isize>)> {
        let mut current_node = start;
        let mut cost = 0;
        let mut current_distance = 0;
        while current_distance < distance {
            current_node = current_node.get_point_in_direction(direction, 1);
            current_distance += 1;
            if let Some(c) = self.grid.get(&current_node) {
                cost += c;
            } else {
                return None;
            }
        }
        Some((cost, current_node))
    }

    fn _print_with_path(&self, path: HashMap<Point2<isize>, Point2Direction>) {
        let (min, max) = self.grid.dimensions();
        println!("City ({}, {}):", min, max);
        for y in min.0[1]..(max.0[1] + 1) {
            for x in min.0[0]..(max.0[0] + 1) {
                let pos = &Point2::new(x, y);
                if let Some(dir) = path.get(pos) {
                    print!("{}", dir);
                } else if let Some(u) = self.grid.get(pos) {
                    print!("{}", u);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }
}

impl From<&str> for City {
    fn from(value: &str) -> Self {
        let mut grid = PointGrid::default();
        for (y, row) in value.trim().lines().enumerate() {
            for (x, c) in row.char_indices() {
                let v: u32 = c.to_string().parse::<u32>().unwrap();
                grid.insert(Point2::new(x as isize, y as isize), v);
            }
        }
        Self { grid }
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let city = City::from(_input);
    let (min, max) = city.grid.dimensions();
    if let Some((cost, _)) = city.cubicle_dijkstra(min, max) {
        return Some(cost);
    }

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    let city = City::from(_input);
    let (min, max) = city.grid.dimensions();
    if let Some((cost, _)) = city.ultra_cubicle_dijkstra(min, max) {
        return Some(cost);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(102));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(94));
    }
}
