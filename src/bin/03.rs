use advent_of_code::algebra_helpers::Point2;
use itertools::Itertools;

#[derive(Debug)]
struct NumberOnMap {
    value: u32,
    start_position: Point2<isize>,
    end_position: Point2<isize>,
    symbol: Option<Symbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Symbol {
    value: char,
    position: Point2<isize>,
}

impl NumberOnMap {
    fn is_part_number(&self, symbols: &[Symbol]) -> Option<Symbol> {
        for s in symbols {
            if s.position.0[1] < self.start_position.0[1] - 1
                || s.position.0[1] > self.start_position.0[1] + 1
            {
                continue;
            } else if s.position.0[0] < self.start_position.0[0] - 1
                || s.position.0[0] > self.end_position[0] + 1
            {
                continue;
            }
            return Some(*s);
        }
        None
    }
}

fn parse_map(s: &str) -> (Vec<NumberOnMap>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for (y, row) in s.lines().enumerate() {
        let mut skip_numbers = false;
        for (x, c) in row.chars().enumerate() {
            if skip_numbers && c.is_numeric() {
                continue;
            } else {
                skip_numbers = false;
            }

            match c {
                '.' => continue,
                _ if c.is_numeric() => {
                    let mut constructed_number = "".to_string();
                    let mut position = x;

                    while let Some(next_c) = row.chars().nth(position) {
                        if !next_c.is_numeric() {
                            break;
                        }

                        constructed_number = format!("{}{}", constructed_number, next_c);
                        position += 1;
                        continue;
                    }

                    numbers.push(NumberOnMap {
                        value: constructed_number.parse::<u32>().unwrap(),
                        start_position: Point2::new(x as isize, y as isize),
                        end_position: Point2::new((position - 1) as isize, y as isize),
                        symbol: None,
                    });
                    skip_numbers = true;
                }
                _ => {
                    symbols.push(Symbol {
                        value: c,
                        position: Point2::new(x as isize, y as isize),
                    });
                }
            }
        }
    }

    for n in numbers.iter_mut() {
        let symbol = n.is_part_number(&symbols);
        n.symbol = symbol;
    }

    (numbers, symbols)
}

pub fn part_one(_input: &str) -> Option<u32> {
    let (numbers, _) = parse_map(_input);
    Some(
        numbers
            .iter()
            .filter(|n| n.symbol.is_some())
            .map(|n| n.value)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_map(_input);

    let mut result = 0;
    for s in symbols.iter() {
        if s.value != '*' {
            continue;
        }

        let gear_numbers = numbers
            .iter()
            .filter(|n| n.symbol == Some(*s))
            .map(|n| n.value)
            .collect_vec();

        if gear_numbers.len() != 2 {
            continue;
        }

        result += gear_numbers.iter().product::<u32>();
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(467835));
    }
}
