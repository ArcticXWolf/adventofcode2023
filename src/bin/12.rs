struct ConditionRecord {
    springs: String,
    questionmark_indices: Vec<usize>,
    groupings: Vec<usize>,
}

impl ConditionRecord {
    fn count_ways_to_solve_record(&self) -> usize {
        //println!("------------------------------------------------");
        let mut counter = 0;
        for mapping in 0..(1 << self.questionmark_indices.len()) {
            if self.is_questionmark_mapping_correct(mapping) {
                counter += 1;
            }
        }
        //println!("------------------------------------------------");
        counter
    }

    fn is_questionmark_mapping_correct(&self, mut bitstring: usize) -> bool {
        let mut fixed_record = self.springs.clone();
        let mut i = 0;
        //println!("Trying {} with {}: ", self.springs, bitstring);
        while i < self.questionmark_indices.len() {
            let q_idx = *self.questionmark_indices.get(i).unwrap();
            fixed_record.replace_range(
                q_idx..q_idx + 1,
                if bitstring & 0x1 > 0 { "#" } else { " " },
            );
            bitstring = bitstring >> 1;
            i += 1;
        }

        let result = compare_spring_group_counts(&fixed_record, &self.groupings);
        //println!("       {} {:?}: {}", fixed_record, self.groupings, result);
        result
    }
}

fn compare_spring_group_counts(springs: &String, group_counts: &[usize]) -> bool {
    springs
        .split_whitespace()
        .map(|s| s.len())
        .collect::<Vec<usize>>()
        == group_counts
}

impl TryFrom<&str> for ConditionRecord {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (spring_str, groupings_str) = value.split_once(" ").ok_or("could not split string")?;
        let springs = spring_str.replace(".", " ");
        let questionmark_indices = springs
            .char_indices()
            .filter(|(_, c)| *c == '?')
            .map(|(idx, _)| idx)
            .collect();
        let groupings = groupings_str
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self {
            springs,
            questionmark_indices,
            groupings,
        })
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let records: Vec<ConditionRecord> = _input.lines().map(|l| l.try_into().unwrap()).collect();
    Some(records.iter().map(|r| r.count_ways_to_solve_record()).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparision() {
        assert_eq!(
            compare_spring_group_counts(&"  #  #  ###".to_string(), &[1, 1, 3]),
            true
        );
        assert_eq!(
            compare_spring_group_counts(&"#       ###".to_string(), &[1, 1, 3]),
            false
        );
    }

    #[test]
    fn test_records_comparision() {
        assert_eq!(
            ConditionRecord::try_from("..##.?.##?...# 2,1,3,1")
                .unwrap()
                .is_questionmark_mapping_correct(0b11),
            true
        );
        assert_eq!(
            ConditionRecord::try_from("..##.?.##?...# 2,1,3,1")
                .unwrap()
                .is_questionmark_mapping_correct(0b10),
            false
        );
    }

    #[test]
    fn test_records_counting() {
        assert_eq!(
            ConditionRecord::try_from("..##.?.##?...# 2,1,3,1")
                .unwrap()
                .count_ways_to_solve_record(),
            1
        );
        assert_eq!(
            ConditionRecord::try_from(".??..??...?##. 1,1,3")
                .unwrap()
                .count_ways_to_solve_record(),
            4
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
