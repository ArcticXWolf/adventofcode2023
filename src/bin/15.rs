use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug)]
enum Instruction {
    // (hash, label[, focal length])
    Dash(usize, String),
    Equals(usize, String, usize),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.contains('-') {
            let (label, _) = value.split_once('-').unwrap();
            let hash_value = hash(label);
            return Self::Dash(hash_value, label.to_string());
        } else if value.contains('=') {
            let (label, focal_length) = value.split_once('=').unwrap();
            let hash_value = hash(label);
            return Self::Equals(hash_value, label.to_string(), focal_length.parse().unwrap());
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: String, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

#[derive(Debug)]
struct Room([Vec<Lens>; 256]);

impl Default for Room {
    fn default() -> Self {
        //Self(std::iter::repeat(vec![]).take(256).collect::<Vec<_>>())
        Self(vec![Vec::new(); 256].try_into().expect("static"))
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (box_id, b) in self.0.iter().enumerate() {
            if !b.is_empty() {
                write!(f, "Box {}: ", box_id)?;
                for l in b {
                    write!(f, "{}", l)?;
                }
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

impl Room {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Dash(hash_value, label) => {
                if let Some(index) = self.0[hash_value]
                    .iter()
                    .position(|lens| lens.label == label)
                {
                    self.0[hash_value].remove(index);
                }
            }
            Instruction::Equals(hash_value, label, focal_length) => {
                if let Some(index) = self.0[hash_value]
                    .iter()
                    .position(|lens| lens.label == label)
                {
                    self.0[hash_value][index] = Lens::new(label, focal_length);
                } else {
                    self.0[hash_value].push(Lens::new(label, focal_length));
                }
            }
        }
    }

    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for (bidx, b) in self.0.iter().enumerate() {
            for (lidx, l) in b.iter().enumerate() {
                sum += (bidx + 1) * (lidx + 1) * l.focal_length;
            }
        }
        sum
    }
}

fn hash(s: &str) -> usize {
    let mut acc = 0;

    for c in s.chars() {
        acc += c as usize;
        acc *= 17;
        acc %= 256
    }

    acc
}

pub fn part_one(_input: &str) -> Option<usize> {
    let init_seq = _input.trim().split(',').collect_vec();
    Some(init_seq.iter().map(|&s| hash(s)).sum())
}

pub fn part_two(_input: &str) -> Option<usize> {
    let init_seq = _input
        .trim()
        .split(',')
        .map(Instruction::from)
        .collect_vec();
    let mut room = Room::default();

    for i in init_seq.into_iter() {
        room.execute(i);
    }

    Some(room.focusing_power())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(1320));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(145));
    }
}
