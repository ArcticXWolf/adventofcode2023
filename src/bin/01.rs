use itertools::Itertools;

pub fn part_one(_input: &str) -> Option<u32> {
    let numbers = _input
        .lines()
        .filter_map(|l| {
            let number_chars = l
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec();
            Some(number_chars.iter().nth(0)? * 10 + number_chars.iter().nth_back(0)?)
        })
        .collect_vec();

    Some(numbers.iter().sum())
}

fn pull_numbers(s: &str) -> Vec<u32> {
    s.char_indices()
        .filter_map(|(i, c)| {
            if c.is_numeric() {
                return c.to_string().parse::<u32>().ok();
            }
            match s.split_at(i).1 {
                subs if subs.starts_with("zero") => return Some(0),
                subs if subs.starts_with("one") => return Some(1),
                subs if subs.starts_with("two") => return Some(2),
                subs if subs.starts_with("three") => return Some(3),
                subs if subs.starts_with("four") => return Some(4),
                subs if subs.starts_with("five") => return Some(5),
                subs if subs.starts_with("six") => return Some(6),
                subs if subs.starts_with("seven") => return Some(7),
                subs if subs.starts_with("eight") => return Some(8),
                subs if subs.starts_with("nine") => return Some(9),
                _ => return None,
            }
        })
        .collect_vec()
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(
        _input
            .lines()
            .filter_map(|l| {
                let nv = pull_numbers(l);
                Some(nv.iter().nth(0)? * 10 + nv.iter().nth_back(0)?)
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(209));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(281));
    }
}
