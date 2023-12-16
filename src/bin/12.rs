use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct ConditionRecord {
    springs: u128,
    questionmarks: u128,
    groupings: Vec<usize>,
    cache: HashMap<(u128, u128, Vec<usize>), usize>,
}

impl ConditionRecord {
    fn from(s: &str, expansion_faktor: usize) -> Self {
        let (spring_str, groupings_str) =
            s.split_once(' ').ok_or("could not split string").unwrap();

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
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        groupings = groupings.repeat(expansion_faktor);

        Self {
            springs,
            questionmarks,
            groupings,
            cache: HashMap::new(),
        }
    }

    fn count_ways_to_solve_record_recursively(&mut self) -> usize {
        self._count_ways_to_solve_record_recursively(
            self.springs,
            self.questionmarks,
            &self.groupings.clone(),
        )
    }

    fn _count_ways_to_solve_record_recursively(
        &mut self,
        springs: u128,
        questionmarks: u128,
        groupings: &[usize],
    ) -> usize {
        if groupings.is_empty() {
            if springs == 0 {
                return 1;
            }
            return 0;
        } else if springs | questionmarks == 0 {
            return 0;
        }

        if self
            .cache
            .contains_key(&(springs, questionmarks, groupings.to_vec()))
        {
            return *self
                .cache
                .get(&(springs, questionmarks, groupings.to_vec()))
                .unwrap();
        }

        if (springs | questionmarks) & 0b1 == 0 {
            let result = self._count_ways_to_solve_record_recursively(
                springs >> 1,
                questionmarks >> 1,
                groupings,
            );
            self.cache
                .insert((springs, questionmarks, groupings.to_vec()), result);
            return result;
        }

        if springs & 0b1 == 1 {
            let group = groupings.first().unwrap();
            let is_group_matched =
                (springs | questionmarks) & ((1 << group) - 1) == ((1 << group) - 1);
            let has_group_a_seperator_in_front =
                (springs & (1 << group) == 0) || (questionmarks & (1 << group) > 0);
            if is_group_matched && has_group_a_seperator_in_front {
                let result = self._count_ways_to_solve_record_recursively(
                    springs >> (group + 1),
                    questionmarks >> (group + 1),
                    &groupings[1..],
                );
                self.cache
                    .insert((springs, questionmarks, groupings.to_vec()), result);
                return result;
            }
            self.cache
                .insert((springs, questionmarks, groupings.to_vec()), 0);
            return 0;
        }

        if questionmarks & 0b1 == 1 {
            let result = self._count_ways_to_solve_record_recursively(
                springs | 0b1,
                questionmarks & (u128::MAX - 1),
                groupings,
            ) + self._count_ways_to_solve_record_recursively(
                springs & (u128::MAX - 1),
                questionmarks & (u128::MAX - 1),
                groupings,
            );
            self.cache
                .insert((springs, questionmarks, groupings.to_vec()), result);
            return result;
        }

        unreachable!()
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let records: Vec<ConditionRecord> = _input
        .lines()
        .map(|l| ConditionRecord::from(l, 1))
        .collect();

    let mut sum = 0;
    for mut r in records.into_iter() {
        sum += r.count_ways_to_solve_record_recursively();
    }

    Some(sum)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let records: Vec<ConditionRecord> = _input
        .lines()
        .map(|l| ConditionRecord::from(l, 5))
        .collect();
    let mut sum = 0;
    for mut r in records.into_iter() {
        sum += r.count_ways_to_solve_record_recursively();
    }

    Some(sum)
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
    fn test_records_counting() {
        assert_eq!(
            ConditionRecord::from("..##.?.##?...# 2,1,3,1", 1)
                .count_ways_to_solve_record_recursively(),
            1
        );
        assert_eq!(
            ConditionRecord::from(".??..??...?##. 1,1,3", 1)
                .count_ways_to_solve_record_recursively(),
            4
        );
        assert_eq!(
            ConditionRecord::from("???.### 1,1,3", 5).count_ways_to_solve_record_recursively(),
            1
        );
        assert_eq!(
            ConditionRecord::from(".??..??...?##. 1,1,3", 5)
                .count_ways_to_solve_record_recursively(),
            16384
        );
        assert_eq!(
            ConditionRecord::from("?#?#?#?#?#?#?#? 1,3,1,6", 5)
                .count_ways_to_solve_record_recursively(),
            1
        );
        assert_eq!(
            ConditionRecord::from("????.#...#... 4,1,1", 5)
                .count_ways_to_solve_record_recursively(),
            16
        );
        assert_eq!(
            ConditionRecord::from("????.######..#####. 1,6,5", 5)
                .count_ways_to_solve_record_recursively(),
            2500
        );
        assert_eq!(
            ConditionRecord::from("?###???????? 3,2,1", 5).count_ways_to_solve_record_recursively(),
            506250
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
        assert_eq!(part_two(&input), Some(525152));
    }
}
