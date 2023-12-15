use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug)]
struct Pattern {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        let mut rows = vec![];
        let mut cols = vec![];

        for l in value.lines() {
            let mut bitstring = 0;
            for (idx, c) in l.chars().enumerate() {
                if c == '#' {
                    bitstring |= 1 << idx;
                }
            }
            rows.push(bitstring);
        }
        for i in 0..value.lines().nth(0).unwrap().chars().count() {
            let mut bitstring = 0;
            for (idx, l) in value.lines().enumerate() {
                if l.chars().nth(i).unwrap() == '#' {
                    bitstring |= 1 << idx;
                }
            }
            cols.push(bitstring);
        }

        Self { rows, cols }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.rows {
            writeln!(f, "{:020b}", r)?;
        }
        writeln!(f, "")
    }
}

impl Pattern {
    fn fix_smidge_and_rank(&self) -> usize {
        for ((r1_idx, r1), (r2_idx, r2)) in
            self.rows.clone().iter().enumerate().tuple_combinations()
        {
            if (r1 ^ r2).count_ones() == 1 {
                let smidge_creates_reflection_oneway = self.is_reflection(
                    (r2_idx - r1_idx) / 2 + r1_idx + 1,
                    true,
                    true,
                    r1_idx,
                    r1 ^ (r1 ^ r2),
                );
                let smidge_create_reflection_otherway = self.is_reflection(
                    (r2_idx - r1_idx) / 2 + r1_idx + 1,
                    true,
                    true,
                    r2_idx,
                    r2 ^ (r1 ^ r2),
                );
                if smidge_creates_reflection_oneway && smidge_create_reflection_otherway {
                    return ((r2_idx - r1_idx) / 2 + r1_idx + 1) * 100;
                }
            }
        }
        for ((c1_idx, c1), (c2_idx, c2)) in
            self.cols.clone().iter().enumerate().tuple_combinations()
        {
            if (c1 ^ c2).count_ones() == 1 {
                let smidge_creates_reflection_oneway = self.is_reflection(
                    (c2_idx - c1_idx) / 2 + c1_idx + 1,
                    false,
                    true,
                    c1_idx,
                    c1 ^ (c1 ^ c2),
                );
                let smidge_create_reflection_otherway = self.is_reflection(
                    (c2_idx - c1_idx) / 2 + c1_idx + 1,
                    false,
                    true,
                    c2_idx,
                    c2 ^ (c1 ^ c2),
                );
                if smidge_creates_reflection_oneway && smidge_create_reflection_otherway {
                    return (c2_idx - c1_idx) / 2 + c1_idx + 1;
                }
            }
        }
        return 0;
    }

    fn find_reflection_rank(&self) -> usize {
        for split_idx in 1..self.rows.len() {
            if self.is_reflection(split_idx, true, false, 0, 0) {
                return split_idx * 100;
            }
        }
        for split_idx in 1..self.cols.len() {
            if self.is_reflection(split_idx, false, false, 0, 0) {
                return split_idx;
            }
        }
        return 0;
    }

    fn is_reflection(
        &self,
        split_idx: usize,
        horizontal: bool,
        replace: bool,
        replace_idx: usize,
        replace_value: usize,
    ) -> bool {
        let compare_against = match horizontal {
            true => &self.rows,
            false => &self.cols,
        };
        let mut compare_offset = 0;
        while (split_idx as isize - compare_offset as isize - 1) >= 0
            && (split_idx + compare_offset) < compare_against.len()
        {
            let left = if replace && replace_idx == split_idx - compare_offset - 1 {
                replace_value
            } else {
                *compare_against.get(split_idx - compare_offset - 1).unwrap()
            };
            let right = if replace && replace_idx == split_idx + compare_offset {
                replace_value
            } else {
                *compare_against.get(split_idx + compare_offset).unwrap()
            };
            if left != right {
                return false;
            }
            compare_offset += 1;
        }
        return true;
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let patterns = _input
        .trim()
        .split("\n\n")
        .map(|s| Pattern::from(s))
        .collect_vec();

    Some(patterns.iter().map(|p| p.find_reflection_rank()).sum())
}

pub fn part_two(_input: &str) -> Option<usize> {
    let patterns = _input
        .trim()
        .split("\n\n")
        .map(|s| Pattern::from(s))
        .collect_vec();

    Some(patterns.iter().map(|p| p.fix_smidge_and_rank()).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(405));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(400));
    }
}
