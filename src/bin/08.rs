use std::collections::HashMap;

use advent_of_code::helpers::lcm_mn;
use itertools::Itertools;

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

        Self { 0: hm }
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
}

#[derive(Debug)]
struct CycleDetector {
    path_tracer: Vec<Vec<(usize, String)>>,
    found_cycles: Vec<Option<(usize, usize)>>, // (cycle_lenth, offset)
    ending_state_offsets: Vec<usize>,
}

impl CycleDetector {
    fn new(starting_nodes: &[&str]) -> Self {
        let mut path_tracer = vec![];
        let mut found_cycles = vec![];
        let mut ending_state_offsets = vec![];

        for _ in starting_nodes {
            path_tracer.push(vec![]);
            found_cycles.push(None);
            ending_state_offsets.push(0);
        }

        Self {
            path_tracer,
            found_cycles,
            ending_state_offsets,
        }
    }

    fn detect_cycles(&mut self, current_nodes: &[&str], instruction_cycle_offset: usize) -> bool {
        for (idx, c) in current_nodes.iter().enumerate() {
            if self.found_cycles[idx].is_some() {
                continue;
            }

            if self.path_tracer[idx].contains(&(instruction_cycle_offset, c.to_string())) {
                let cycle_offset = self.path_tracer[idx]
                    .iter()
                    .position(|e| *e == (instruction_cycle_offset, c.to_string()))
                    .unwrap();
                let cycle_length = self.path_tracer[idx].len() - cycle_offset;
                self.found_cycles[idx] = Some((cycle_length, cycle_offset));
            } else {
                if c.ends_with("Z") {
                    self.ending_state_offsets[idx] = self.path_tracer[idx].len();
                }
                self.path_tracer[idx].push((instruction_cycle_offset, c.to_string()));
            }
        }

        self.found_cycles.iter().all(|c| c.is_some())
    }
}

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

pub fn part_one(_input: &str) -> Option<u32> {
    let (instructions, map) = parser::parse(_input.trim()).unwrap();

    let mut current_node = "AAA";
    let mut instruction_iterator = instructions.iter().cycle();
    let mut counter = 0;

    while current_node != "ZZZ" {
        current_node = map.go_instruction(current_node, instruction_iterator.next().unwrap());
        counter += 1;
    }

    Some(counter)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (instructions, map) = parser::parse(_input.trim()).unwrap();

    let mut current_nodes = map
        .0
        .keys()
        .map(|s| s.as_str())
        .filter(|s| s.ends_with("A"))
        .sorted()
        .collect_vec();
    let mut instruction_iterator = instructions.iter().enumerate().cycle();
    let mut cycle_detector = CycleDetector::new(&current_nodes);

    loop {
        let (instruction_cycle_idx, instruction) = instruction_iterator.next().unwrap();
        current_nodes = current_nodes
            .iter()
            .map(|c| map.go_instruction(c, instruction))
            .collect_vec();

        if cycle_detector.detect_cycles(&current_nodes, instruction_cycle_idx) {
            break;
        }
    }

    Some(lcm_mn(
        &cycle_detector
            .found_cycles
            .iter()
            .map(|c| c.unwrap().0)
            .collect_vec(),
    ) as u64)
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
