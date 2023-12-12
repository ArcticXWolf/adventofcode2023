#[derive(Debug, PartialEq, Eq)]
struct ConditionRecord {
    springs: u128,
    questionmarks: u128,
    groupings: Vec<usize>,
}

impl ConditionRecord {
    fn from(s: &str, expansion_faktor: usize) -> Self {
        let (spring_str, groupings_str) =
            s.split_once(" ").ok_or("could not split string").unwrap();

        let mut springs_pre = 0;
        let mut questionmarks_pre = 0;

        for (idx, c) in spring_str.char_indices() {
            match c {
                '#' => {
                    springs_pre |= 1 << idx;
                }
                '?' => {
                    questionmarks_pre |= 1 << idx;
                }
                _ => {}
            }
        }

        let mut springs = springs_pre;
        let mut questionmarks = questionmarks_pre;
        for i in 1..expansion_faktor {
            springs |= (springs_pre << 1) << ((spring_str.len() + 1) * i - 1);
            questionmarks |= ((questionmarks_pre << 1) | 0b1) << ((spring_str.len() + 1) * i - 1);
        }

        let mut groupings = groupings_str
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        let groups = spring_str
            .replace(".", " ")
            .split_whitespace()
            .map(|s| s.len())
            .collect::<Vec<usize>>();
        println!(
            "G C{:?} P{:?} - {:?} {:?}",
            groups.len(),
            groupings.len(),
            groups,
            groupings
        );

        groupings = groupings.repeat(expansion_faktor);

        Self {
            springs,
            questionmarks,
            groupings,
        }
    }

    fn count_ways_to_solve_record(&self) -> usize {
        let mut mapping = !self.questionmarks;
        let mut counter = 0;
        for _ in 0..(1 << self.questionmarks.count_ones()) {
            if self.is_questionmark_mapping_correct(mapping) {
                counter += 1;
            }
            mapping = ((mapping.wrapping_add(1)) & self.questionmarks) | (!self.questionmarks);
        }
        counter
    }

    fn is_questionmark_mapping_correct(&self, bitstring: u128) -> bool {
        compare_spring_group_counts(
            (self.springs & (!self.questionmarks)) | (bitstring & self.questionmarks),
            &self.groupings,
        )
    }
}

fn compare_spring_group_counts(springs: u128, group_counts: &[usize]) -> bool {
    let mut bitstring = springs.clone();
    let mut groups_detected = vec![];
    let mut counter = 0;

    while bitstring > 0 {
        if bitstring & 0b1 == 0 && counter > 0 {
            groups_detected.push(counter);
            counter = 0;
        } else if bitstring & 0b1 > 0 {
            counter += 1;
        }

        bitstring = bitstring >> 1;
    }
    if counter > 0 {
        groups_detected.push(counter);
    }

    groups_detected == group_counts
}

pub fn part_one(_input: &str) -> Option<usize> {
    let records: Vec<ConditionRecord> = _input
        .lines()
        .map(|l| ConditionRecord::from(l, 1))
        .collect();
    Some(records.iter().map(|r| r.count_ways_to_solve_record()).sum())
}

pub fn part_two(_input: &str) -> Option<usize> {
    let records: Vec<ConditionRecord> = _input
        .lines()
        .map(|l| ConditionRecord::from(l, 5))
        .collect();
    Some(records.iter().map(|r| r.count_ways_to_solve_record()).sum())
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
            compare_spring_group_counts(0b011101000010, &[1, 1, 3]),
            true
        );
        assert_eq!(compare_spring_group_counts(0b01100111, &[1, 1, 3]), false);
    }

    #[test]
    fn test_records() {
        assert_eq!(
            ConditionRecord::from(".??..??...?##. 1,1,3", 1),
            ConditionRecord {
                springs: 0b1100000000000,
                questionmarks: 0b10001100110,
                groupings: vec![1, 1, 3]
            }
        );
        assert_eq!(
            ConditionRecord::from(".# 1", 1),
            ConditionRecord {
                springs: 0b10,
                questionmarks: 0b0,
                groupings: vec![1]
            }
        );
        assert_eq!(
            ConditionRecord::from(".# 1", 5),
            ConditionRecord {
                springs: 0b10010010010010,
                questionmarks: 0b100100100100,
                groupings: vec![1, 1, 1, 1, 1]
            }
        );
        assert_eq!(
            ConditionRecord::from("???.### 1,1,3", 1),
            ConditionRecord {
                springs: 0b1110000,
                questionmarks: 0b0000111,
                groupings: vec![1, 1, 3]
            }
        );
        assert_eq!(
            ConditionRecord::from("???.### 1,1,3", 5),
            ConditionRecord {
                springs: 0b111000001110000011100000111000001110000,
                questionmarks: 0b000011110000111100001111000011110000111,
                groupings: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            }
        );
    }

    #[test]
    fn test_records_comparision() {
        assert_eq!(
            ConditionRecord::from("..##.?.##?...# 2,1,3,1", 1)
                .is_questionmark_mapping_correct(0b1000100000),
            true
        );
        assert_eq!(
            ConditionRecord::from("..##.?.##?...# 2,1,3,1", 1)
                .is_questionmark_mapping_correct(0b10),
            false
        );
    }

    #[test]
    fn test_records_counting() {
        assert_eq!(
            ConditionRecord::from("..##.?.##?...# 2,1,3,1", 1).count_ways_to_solve_record(),
            1
        );
        assert_eq!(
            ConditionRecord::from(".??..??...?##. 1,1,3", 1).count_ways_to_solve_record(),
            4
        );
        assert_eq!(
            ConditionRecord::from("???.### 1,1,3", 5).count_ways_to_solve_record(),
            1
        );
        assert_eq!(
            ConditionRecord::from(".??..??...?##. 1,1,3", 5).count_ways_to_solve_record(),
            16384
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
        assert_eq!(part_two(&input), Some(506250));
    }
}
