use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct GardeningTranslationRange {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

impl GardeningTranslationRange {
    fn translate(&self, value: i64) -> Option<i64> {
        if (self.source_start..self.source_end).contains(&value) {
            return Some(value + self.offset);
        }
        None
    }

    fn translate_range(&self, value: Range) -> Option<Range> {
        if (self.source_start..self.source_end).contains(&value.from) {
            return Some(Range::new(value.from + self.offset, value.to + self.offset));
        }
        None
    }
}

impl From<(i64, i64, i64)> for GardeningTranslationRange {
    fn from(value: (i64, i64, i64)) -> Self {
        Self {
            source_start: value.1,
            source_end: value.1 + value.2,
            offset: value.0 - value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GardeningMap {
    _source_tag: String,
    _destination_tag: String,
    ranges: Vec<GardeningTranslationRange>,
}

impl GardeningMap {
    fn translate(&self, value: i64) -> i64 {
        for range in self.ranges.iter() {
            if let Some(tv) = range.translate(value) {
                return tv;
            }
        }
        value
    }

    fn translate_range(&self, value: &Range) -> Range {
        for range in self.ranges.iter() {
            if let Some(tv) = range.translate_range(value.clone()) {
                return tv;
            }
        }
        value.clone()
    }
}

impl From<((&str, &str), Vec<GardeningTranslationRange>)> for GardeningMap {
    fn from(value: ((&str, &str), Vec<GardeningTranslationRange>)) -> Self {
        Self {
            _source_tag: value.0 .0.to_string(),
            _destination_tag: value.0 .1.to_string(),
            ranges: value.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    from: i64,
    to: i64,
}

impl Range {
    fn new(from: i64, to: i64) -> Self {
        Self { from, to }
    }

    fn from_seedlist(seeds: Vec<i64>) -> Vec<Self> {
        seeds
            .into_iter()
            .tuples::<(_, _)>()
            .map(|(from, length)| Self::new(from, from + length))
            .collect_vec()
    }

    fn split_according_to_map(&self, map: &GardeningMap) -> Vec<Self> {
        let mut new_ranges = vec![];

        let mut splitpoints: HashSet<i64> = HashSet::new();
        for m in &map.ranges {
            splitpoints.insert(m.source_start);
            splitpoints.insert(m.source_end);
        }

        let mut current = self.from;
        for sp in splitpoints.into_iter().sorted() {
            if sp < current {
                continue;
            }
            if sp > self.to {
                break;
            }
            new_ranges.push(Self::new(current, sp));
            current = sp;
        }
        if current < self.to {
            new_ranges.push(Self::new(current, self.to));
        }

        new_ranges
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, i64, line_ending, space1},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{pair, preceded, separated_pair, terminated, tuple},
        Finish, IResult,
    };

    use crate::{GardeningMap, GardeningTranslationRange};

    pub fn parse_instructions(s: &str) -> Result<(Vec<i64>, Vec<GardeningMap>), NomError<&str>> {
        let (_, x) =
            separated_pair(parse_seeds, pair(line_ending, line_ending), parse_maps)(s).finish()?;
        Ok(x)
    }

    fn parse_seeds(s: &str) -> IResult<&str, Vec<i64>> {
        preceded(tag("seeds: "), separated_list1(space1, i64))(s)
    }

    fn parse_maps(s: &str) -> IResult<&str, Vec<GardeningMap>> {
        separated_list1(pair(line_ending, line_ending), parse_map)(s)
    }

    fn parse_map(s: &str) -> IResult<&str, GardeningMap> {
        into(parse_map_raw)(s)
    }

    fn parse_map_raw(s: &str) -> IResult<&str, ((&str, &str), Vec<GardeningTranslationRange>)> {
        separated_pair(parse_tags, line_ending, parse_ranges)(s)
    }

    fn parse_tags(s: &str) -> IResult<&str, (&str, &str)> {
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(s)
    }

    fn parse_ranges(s: &str) -> IResult<&str, Vec<GardeningTranslationRange>> {
        separated_list1(line_ending, parse_range)(s)
    }

    fn parse_range(s: &str) -> IResult<&str, GardeningTranslationRange> {
        into(parse_range_raw)(s)
    }

    fn parse_range_raw(s: &str) -> IResult<&str, (i64, i64, i64)> {
        tuple((terminated(i64, space1), terminated(i64, space1), i64))(s)
    }
}

pub fn part_one(_input: &str) -> Option<i64> {
    let (seeds, maps) = parser::parse_instructions(_input).unwrap();

    let translated_seeds = seeds
        .iter()
        .map(|s| {
            let mut ts = *s;
            for map in maps.iter() {
                ts = map.translate(ts);
            }
            ts
        })
        .collect_vec();
    Some(*translated_seeds.iter().min().unwrap())
}

pub fn part_two(_input: &str) -> Option<i64> {
    let (seeds, maps) = parser::parse_instructions(_input).unwrap();

    let mut rangelist = Range::from_seedlist(seeds);

    for m in maps {
        let mut new_rangelist: Vec<Range> = vec![];
        for r in &rangelist {
            let split_ranges = r.split_according_to_map(&m);
            let translated_ranges = split_ranges
                .iter()
                .map(|sr| m.translate_range(sr))
                .collect_vec();
            new_rangelist.extend(translated_ranges);
        }
        rangelist = new_rangelist;
    }

    rangelist.iter().map(|r| r.from).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(46));
    }
}
