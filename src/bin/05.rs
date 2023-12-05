use itertools::Itertools;

// Part2 is bruteforcable in ~2min in Rust. I believe there is a nice performant
// solution by combining the maps into one map (see GardeningMap.combine_with_map()).
// If I do a performance refactor, then I will add this.

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

    fn combine_with_map(self, other: GardeningMap) -> GardeningMap {
        // FIX: this code will split ranges, but just add up the offsets
        // This is obviously wrong. Will fix this, if I do a performance refactoring.
        let mut range_points = vec![];
        for r in &self.ranges {
            range_points.push((r.source_start, true, r.offset));
            range_points.push((r.source_end, false, r.offset));
        }
        for r in &other.ranges {
            range_points.push((r.source_start, true, r.offset));
            range_points.push((r.source_end, false, r.offset));
        }
        range_points.sort();

        let mut result_ranges = vec![];
        let mut last_start = range_points.first().unwrap().0;
        let mut current_offset = range_points.first().unwrap().2;
        for (p, is_start, offset) in range_points.iter().skip(1) {
            if *is_start {
                if *p > last_start {
                    result_ranges.push(GardeningTranslationRange {
                        source_start: last_start,
                        source_end: *p,
                        offset: current_offset,
                    });
                }
                current_offset += offset;
                last_start = *p;
            } else {
                if *p > last_start {
                    result_ranges.push(GardeningTranslationRange {
                        source_start: last_start,
                        source_end: *p,
                        offset: current_offset,
                    });
                }
                current_offset -= offset;
                last_start = *p;
            }
        }

        GardeningMap {
            _source_tag: self._source_tag.clone(),
            _destination_tag: other._destination_tag.clone(),
            ranges: result_ranges,
        }
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

    /*  let combined_map = maps
          .into_iter()
          .reduce(|m, o| m.combine_with_map(o))
          .unwrap();
    */

    let translated_seeds = seeds
        .iter()
        .tuples::<(_, _)>()
        .flat_map(|(s_start, s_length)| (*s_start..(s_start + s_length)).collect_vec())
        .map(|s| {
            let mut ts = s;
            for map in maps.iter() {
                ts = map.translate(ts);
            }
            ts
        })
        .collect_vec();
    Some(*translated_seeds.iter().min().unwrap())
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

    #[test]
    fn test_combine_maps() {
        let map1 = GardeningMap {
            _source_tag: "test".to_string(),
            _destination_tag: "bla".to_string(),
            ranges: vec![GardeningTranslationRange {
                source_start: 10,
                source_end: 20,
                offset: 10,
            }],
        };
        let map2 = GardeningMap {
            _source_tag: "bla".to_string(),
            _destination_tag: "blubb".to_string(),
            ranges: vec![GardeningTranslationRange {
                source_start: 15,
                source_end: 25,
                offset: -20,
            }],
        };

        assert_eq!(
            map1.combine_with_map(map2),
            GardeningMap {
                _source_tag: "test".to_string(),
                _destination_tag: "blubb".to_string(),
                ranges: vec![
                    GardeningTranslationRange {
                        source_start: 10,
                        source_end: 15,
                        offset: 10,
                    },
                    GardeningTranslationRange {
                        source_start: 15,
                        source_end: 20,
                        offset: -10,
                    },
                    GardeningTranslationRange {
                        source_start: 20,
                        source_end: 25,
                        offset: -20,
                    },
                ],
            }
        );
    }
}
