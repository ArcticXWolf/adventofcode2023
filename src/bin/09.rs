#![feature(iter_map_windows)]
use itertools::Itertools;

// part2 = part1 with numberlist reversed :D
// Idea for refactor: do not compute via rate of changes, but interpolate
// base function directly and sample from it

fn parse_input(s: &str) -> Vec<Vec<i64>> {
    s.trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn construct_rate_of_changes(numbers: &[i64]) -> Vec<Vec<i64>> {
    let mut rates_of_change = vec![numbers.to_vec()];

    // while last roc is all 0
    while rates_of_change.last().unwrap().iter().all_equal_value() != Ok(&0) {
        // create pairwise difference
        let new_roc = rates_of_change
            .last()
            .unwrap()
            .iter()
            .map_windows(|[&a, &b]| b - a)
            .collect_vec();

        rates_of_change.push(new_roc);
    }

    rates_of_change
}

fn interpolate_number(numbers_with_roc: &[Vec<i64>]) -> i64 {
    let mut previous_change = *numbers_with_roc.last().unwrap().last().unwrap();
    for i in (0..(numbers_with_roc.len() - 1)).rev() {
        let current_last_element = *numbers_with_roc.get(i).unwrap().last().unwrap();
        previous_change += current_last_element;
    }

    previous_change
}

pub fn part_one(_input: &str) -> Option<i64> {
    let metrics = parse_input(_input);

    let mut interpolated_numbers = vec![];
    for m in metrics {
        let mut rates_of_change = construct_rate_of_changes(&m);
        rates_of_change.last_mut().unwrap().push(0);

        interpolated_numbers.push(interpolate_number(&rates_of_change));
    }

    Some(interpolated_numbers.iter().sum())
}

pub fn part_two(_input: &str) -> Option<i64> {
    let metrics = parse_input(_input);

    let mut interpolated_numbers = vec![];
    for mut m in metrics {
        // flip em around!
        m.reverse();
        let mut rates_of_change = construct_rate_of_changes(&m);
        rates_of_change.last_mut().unwrap().push(0);

        interpolated_numbers.push(interpolate_number(&rates_of_change));
    }

    Some(interpolated_numbers.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(114));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(2));
    }
}
