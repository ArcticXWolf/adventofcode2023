use std::collections::HashSet;

use advent_of_code::algebra_helpers::{Cube, Point, Point3};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick(Cube<isize>);

impl Brick {
    fn can_drop(&self, brickstack: &Brickstack) -> bool {
        if self.0.min.0[2] <= 1 || self.0.max.0[2] <= 1 {
            return false;
        }

        let new_brick = Brick(self.0 - Point3::new(0, 0, 1));
        !brickstack
            .0
            .iter()
            .any(|b| b != self && b.0.intersects(&new_brick.0))
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (p1_str, p2_str) = value.split_once('~').unwrap();
        Self(Cube::new(
            parse_point3(p1_str),
            parse_point3(p2_str) + Point::one(),
        ))
    }
}

fn parse_point3(s: &str) -> Point3<isize> {
    let [x, y, z] = s
        .splitn(3, ',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect_vec()[0..3]
    else {
        panic!()
    };
    Point3::new(x, y, z)
}

#[derive(Debug, Clone)]
struct Brickstack(Vec<Brick>);

impl From<&str> for Brickstack {
    fn from(value: &str) -> Self {
        Self(value.trim().lines().map(Brick::from).collect())
    }
}

impl Brickstack {
    fn drop(&mut self) -> usize {
        let mut changed = true;
        let mut bricks_dropped = HashSet::new();

        while changed {
            changed = false;
            let comparing = self.clone();
            for (i, b) in self.0.iter_mut().enumerate() {
                while b.can_drop(&comparing) {
                    b.0.min -= Point3::new(0, 0, 1);
                    b.0.max -= Point3::new(0, 0, 1);
                    bricks_dropped.insert(i);
                    changed = true;
                }
            }
        }

        bricks_dropped.len()
    }

    fn find_disintegratable(&self) -> Vec<usize> {
        let mut result = vec![];
        for probable_disintegrator in &self.0 {
            let mut remaining_stack = self.clone();
            remaining_stack.0 = remaining_stack
                .0
                .into_iter()
                .filter(|&b| b != *probable_disintegrator)
                .collect_vec();
            let amount_of_bricks_dropped = remaining_stack.drop();
            result.push(amount_of_bricks_dropped);
        }

        result
    }

    fn brick_symbol(i: &usize) -> char {
        match *i {
            x @ 0..=60 => (x + 65) as u8 as char,
            _ => '&',
        }
    }

    fn print_xz(&self) {
        let max_x = self.0.iter().map(|b| b.0.max.0[0]).max().unwrap();
        let max_y = self.0.iter().map(|b| b.0.max.0[1]).max().unwrap();
        let max_z = self.0.iter().map(|b| b.0.max.0[2]).max().unwrap();

        println!("XZ-Plane: ");
        for z in (1..max_z).rev() {
            print!("z{:04} #", z);
            'cell: for x in 0..max_x {
                for y in 0..max_y {
                    for (i, b) in self.0.iter().enumerate() {
                        if b.0.contains(&Point3::new(x, y, z)) {
                            print!("{}", Brickstack::brick_symbol(&i));
                            continue 'cell;
                        }
                    }
                }
                print!(".");
            }
            println!("#");
        }
        println!("      #{}#", (0..max_x).map(|_| "#").join(""));
    }

    fn print_yz(&self) {
        let max_x = self.0.iter().map(|b| b.0.max.0[0]).max().unwrap();
        let max_y = self.0.iter().map(|b| b.0.max.0[1]).max().unwrap();
        let max_z = self.0.iter().map(|b| b.0.max.0[2]).max().unwrap();

        println!("YZ-Plane: ");
        for z in (1..max_z).rev() {
            print!("z{:04} #", z);
            'cell: for y in 0..max_y {
                for x in 0..max_x {
                    for (i, b) in self.0.iter().enumerate() {
                        if b.0.contains(&Point3::new(x, y, z)) {
                            print!("{}", Brickstack::brick_symbol(&i));
                            continue 'cell;
                        }
                    }
                }
                print!(".");
            }
            println!("#");
        }
        println!("      #{}#", (0..max_y).map(|_| "#").join(""));
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut brickstack = Brickstack::from(_input);
    brickstack.drop();
    Some(
        brickstack
            .find_disintegratable()
            .iter()
            .filter(|i| **i == 0)
            .count(),
    )
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut brickstack = Brickstack::from(_input);
    brickstack.drop();
    Some(brickstack.find_disintegratable().into_iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(7));
    }
}
