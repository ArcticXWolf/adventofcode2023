use advent_of_code::algebra_helpers::{Point2, PointGrid};
use itertools::Itertools;

// The first time where I was able to anticipate and solve part2 before knowing
// it :D

fn parse_input(s: &str) -> (PointGrid<isize, 2, ()>, Vec<isize>, Vec<isize>) {
    let mut grid = PointGrid::default();

    for (line_idx, line) in s.trim().lines().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c == '#' {
                grid.insert(Point2::new(char_idx as isize, line_idx as isize), ())
            }
        }
    }

    let max_y = s.trim().lines().count();
    let max_x = s.trim().lines().map(|l| l.chars().count()).max().unwrap();
    let empty_line_indices = (0..max_y as isize)
        .filter(|&y| {
            (0..max_x as isize)
                .filter_map(|x| grid.get(&Point2::new(x, y)))
                .count()
                == 0
        })
        .collect_vec();
    let empty_col_indices = (0..max_x as isize)
        .filter(|&x| {
            (0..max_y as isize)
                .filter_map(|y| grid.get(&Point2::new(x, y)))
                .count()
                == 0
        })
        .collect_vec();

    (grid, empty_line_indices, empty_col_indices)
}

pub fn calc_with_galaxy_age(_input: &str, age: isize) -> Option<isize> {
    let (grid, empty_lines, empty_cols) = parse_input(_input);

    let mut path_length = vec![];
    for (p1, p2) in grid.0.keys().tuple_combinations() {
        let empty_lines_in_distance = empty_lines
            .iter()
            .filter(|&el| *el > p1.0[1].min(p2.0[1]) && *el < p1.0[1].max(p2.0[1]))
            .collect_vec();
        let empty_cols_in_distance = empty_cols
            .iter()
            .filter(|&ec| *ec > p1.0[0].min(p2.0[0]) && *ec < p1.0[0].max(p2.0[0]))
            .collect_vec();
        let distance = (p2.0[1] - p1.0[1]).abs()
            + (p2.0[0] - p1.0[0]).abs()
            + (empty_lines_in_distance.len() as isize) * (age - 1)
            + (empty_cols_in_distance.len() as isize) * (age - 1);

        path_length.push(distance);
    }

    Some(path_length.iter().sum())
}

pub fn part_one(_input: &str) -> Option<isize> {
    calc_with_galaxy_age(_input, 2)
}

pub fn part_two(_input: &str) -> Option<isize> {
    calc_with_galaxy_age(_input, 1000000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(calc_with_galaxy_age(&input, 10), Some(1030));
        assert_eq!(calc_with_galaxy_age(&input, 100), Some(8410));
    }
}
