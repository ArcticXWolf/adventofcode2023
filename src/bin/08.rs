use std::collections::HashMap;

use advent_of_code::helpers::lcm_mn;
use itertools::Itertools;

// My initial solution had more complex structs and code like the cycle
// detector.  This brought my performance down significantly and still was no
// general solution for the problem. For example:
//   1. what if there are multiple ending states per cycle?
//   2. what if the ending states are not at the end of a cycle?
//   3. what if the cycle does not begin at the starting state?
// I am sure you can solve this generally by clever adaptations of the lcm
// algorithm, but since the input in question is pretty nice and does not hit
// the above problems I decided to simplify my solution for performance. That is
// why I have no cycle detection and just assume that the path loops after an
// ending state. I did check this property on my input beforehand.  Lastly, the
// code of the proper cycle detection is in the previous commit.

#[derive(Debug)]
pub enum Instruction {
    Right,
    Left,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct NodeMap(HashMap<String, (String, String)>);

impl From<Vec<(&str, (&str, &str))>> for NodeMap {
    fn from(value: Vec<(&str, (&str, &str))>) -> Self {
        let mut hm = HashMap::new();

        for (k, (vl, vr)) in value {
            hm.insert(k.to_string(), (vl.to_string(), vr.to_string()));
        }

        Self(hm)
    }
}

impl NodeMap {
    fn go_instruction(&self, current_node: &str, instruction: &Instruction) -> &str {
        if let Some((l, r)) = self.0.get(current_node) {
            return match instruction {
                Instruction::Left => l.as_str(),
                Instruction::Right => r.as_str(),
            };
        }
        unreachable!()
    }

    fn follow_path(&self, instructions: &[Instruction], starting_node: &str, part2: bool) -> usize {
        let mut current_node = starting_node;
        let mut instruction_iterator = instructions.iter().cycle();
        let mut counter = 0;

        while !current_node.ends_with(if part2 { "Z" } else { "ZZZ" }) {
            current_node = self.go_instruction(current_node, instruction_iterator.next().unwrap());
            counter += 1;
        }

        counter
    }
}

#[allow(clippy::type_complexity)]
mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, char, line_ending},
        combinator::into,
        error::Error as NomError,
        multi::{many1, separated_list1},
        sequence::{delimited, pair, separated_pair},
        Finish, IResult,
    };

    use crate::{Instruction, NodeMap};

    pub fn parse(s: &str) -> Result<(Vec<Instruction>, NodeMap), NomError<&str>> {
        let (_, x) = separated_pair(
            parse_instructions,
            pair(line_ending, line_ending),
            parse_map,
        )(s)
        .finish()?;
        Ok(x)
    }

    fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
        many1(parse_instruction)(s)
    }

    fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
        into(parse_instruction_raw)(s)
    }

    fn parse_instruction_raw(s: &str) -> IResult<&str, char> {
        alt((char('R'), char('L')))(s)
    }

    fn parse_map(s: &str) -> IResult<&str, NodeMap> {
        into(parse_map_raw)(s)
    }

    fn parse_map_raw(s: &str) -> IResult<&str, Vec<(&str, (&str, &str))>> {
        separated_list1(line_ending, parse_map_entry)(s)
    }

    fn parse_map_entry(s: &str) -> IResult<&str, (&str, (&str, &str))> {
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        )(s)
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let (instructions, map) = parser::parse(_input.trim()).unwrap();
    Some(map.follow_path(&instructions, "AAA", false))
}

pub fn part_two(_input: &str) -> Option<usize> {
    let (instructions, map) = parser::parse(_input.trim()).unwrap();

    let starting_nodes = map
        .0
        .keys()
        .map(|s| s.as_str())
        .filter(|s| s.ends_with('A'))
        .sorted()
        .collect_vec();

    let cycles = starting_nodes
        .iter()
        .map(|sn| map.follow_path(&instructions, sn, true))
        .collect_vec();

    Some(lcm_mn(&cycles))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_alternate("examples", 8, 2);
        assert_eq!(part_two(&input), Some(6));
    }
}
