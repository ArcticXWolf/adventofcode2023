use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Unconditional(String),
    XConditionalSmallerThan(usize, String),
    XConditionalBiggerThan(usize, String),
    MConditionalSmallerThan(usize, String),
    MConditionalBiggerThan(usize, String),
    AConditionalSmallerThan(usize, String),
    AConditionalBiggerThan(usize, String),
    SConditionalSmallerThan(usize, String),
    SConditionalBiggerThan(usize, String),
}

impl Rule {
    fn find_redirection(&self, machine_part: &MachinePart) -> Option<String> {
        match self {
            Self::Unconditional(wfn) => Some(wfn.clone()),
            Self::XConditionalSmallerThan(threshold, wfn) => {
                if machine_part.x < *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::XConditionalBiggerThan(threshold, wfn) => {
                if machine_part.x > *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::MConditionalSmallerThan(threshold, wfn) => {
                if machine_part.m < *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::MConditionalBiggerThan(threshold, wfn) => {
                if machine_part.m > *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::AConditionalSmallerThan(threshold, wfn) => {
                if machine_part.a < *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::AConditionalBiggerThan(threshold, wfn) => {
                if machine_part.a > *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::SConditionalSmallerThan(threshold, wfn) => {
                if machine_part.s < *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
            Self::SConditionalBiggerThan(threshold, wfn) => {
                if machine_part.s > *threshold {
                    Some(wfn.clone())
                } else {
                    None
                }
            }
        }
    }

    fn get_redirection_name(&self) -> String {
        match self {
            Self::Unconditional(wfn) => wfn.clone(),
            Self::XConditionalSmallerThan(_, wfn) => wfn.clone(),
            Self::XConditionalBiggerThan(_, wfn) => wfn.clone(),
            Self::MConditionalSmallerThan(_, wfn) => wfn.clone(),
            Self::MConditionalBiggerThan(_, wfn) => wfn.clone(),
            Self::AConditionalSmallerThan(_, wfn) => wfn.clone(),
            Self::AConditionalBiggerThan(_, wfn) => wfn.clone(),
            Self::SConditionalSmallerThan(_, wfn) => wfn.clone(),
            Self::SConditionalBiggerThan(_, wfn) => wfn.clone(),
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if !value.contains(':') {
            Rule::Unconditional(value.to_string())
        } else {
            let (threshold, next_workflow) = value[2..].split_once(':').unwrap();
            match &value[0..2] {
                "x<" => Rule::XConditionalSmallerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "x>" => Rule::XConditionalBiggerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "m<" => Rule::MConditionalSmallerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "m>" => Rule::MConditionalBiggerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "a<" => Rule::AConditionalSmallerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "a>" => Rule::AConditionalBiggerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "s<" => Rule::SConditionalSmallerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                "s>" => Rule::SConditionalBiggerThan(
                    threshold.parse().unwrap(),
                    next_workflow.to_string(),
                ),
                _ => unimplemented!(),
            }
        }
    }
}

#[derive(Debug)]
struct Workflow(Vec<Rule>);

impl Workflow {
    fn find_redirection(&self, machine_part: &MachinePart) -> String {
        for rule in &self.0 {
            if let Some(wfn) = rule.find_redirection(machine_part) {
                return wfn;
            }
        }
        "".to_string()
    }

    fn create_partition_list(&self, partition: Partition) -> Vec<(String, Partition)> {
        let mut partition_list = vec![];
        let mut remaining_partition = partition.clone();
        for rule in &self.0 {
            let (hit, not_hit) = remaining_partition.split_by(rule);
            if let Some(hp) = hit {
                partition_list.push((rule.get_redirection_name(), hp));
            }
            if let Some(nhp) = not_hit {
                remaining_partition = nhp;
            } else {
                break;
            }
        }
        partition_list
    }
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        Self(value.split(',').map(Rule::from).collect())
    }
}

#[derive(Debug)]
struct WorkflowList(HashMap<String, Workflow>);

impl WorkflowList {
    fn is_machine_part_valid(&self, machine_part: &MachinePart) -> bool {
        let mut current_wfn = "in".to_string();

        while current_wfn != "R" && current_wfn != "A" {
            let wf = self.0.get(&current_wfn).unwrap();
            current_wfn = wf.find_redirection(machine_part);
        }

        current_wfn == "A"
    }

    fn create_partition_list(&self) -> Vec<(String, Partition)> {
        let mut current_partition_list = vec![("in".to_string(), Partition::default())];

        while !current_partition_list
            .iter()
            .all(|pl| pl.0 == "A" || pl.0 == "R")
        {
            let mut new_partition_list = vec![];
            for (wfn, partition) in current_partition_list {
                if wfn == "A" || wfn == "R" {
                    new_partition_list.push((wfn, partition));
                    continue;
                }

                if let Some(wf) = self.0.get(&wfn) {
                    new_partition_list.extend(wf.create_partition_list(partition));
                }
                assert!(self.0.get(&wfn).is_some())
            }
            assert_eq!(
                4000_u64 * 4000 * 4000 * 4000,
                new_partition_list
                    .iter()
                    .map(|(_, p)| p.combination_count())
                    .sum()
            );
            current_partition_list = new_partition_list;
        }

        current_partition_list
    }
}

impl From<&str> for WorkflowList {
    fn from(value: &str) -> Self {
        let mut hm = HashMap::default();

        for l in value.lines() {
            let (name, wf_str) = l.split_once('{').unwrap();
            hm.insert(name.to_string(), wf_str.replace('}', "").as_str().into());
        }

        Self(hm)
    }
}

#[derive(Debug, Default)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for MachinePart {
    fn from(value: &str) -> Self {
        let mut part = Self::default();
        for property in value
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let property_value = property[2..].parse().unwrap();
            match property.chars().nth(0) {
                Some('x') => part.x = property_value,
                Some('m') => part.m = property_value,
                Some('a') => part.a = property_value,
                Some('s') => part.s = property_value,
                _ => unimplemented!(),
            }
        }

        part
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Partition {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Default for Partition {
    fn default() -> Self {
        Self {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }
}

impl Partition {
    fn split_by(&self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        match rule {
            Rule::Unconditional(_) => (Some(self.clone()), None),
            Rule::XConditionalSmallerThan(threshold, _) => {
                let hit_by_rule = if self.x.0 < *threshold {
                    Some(Partition {
                        x: (self.x.0, (*threshold).min(self.x.1)),
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    })
                } else {
                    None
                };
                let not_hit = if self.x.1 > *threshold {
                    Some(Partition {
                        x: ((*threshold).max(self.x.0), self.x.1),
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::XConditionalBiggerThan(threshold, _) => {
                let hit_by_rule = if self.x.1 > *threshold + 1 {
                    Some(Partition {
                        x: ((*threshold + 1).max(self.x.0), self.x.1),
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    })
                } else {
                    None
                };
                let not_hit = if self.x.0 < *threshold + 1 {
                    Some(Partition {
                        x: (self.x.0, (*threshold + 1).min(self.x.1)),
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::MConditionalSmallerThan(threshold, _) => {
                let hit_by_rule = if self.m.0 < *threshold {
                    Some(Partition {
                        m: (self.m.0, (*threshold).min(self.m.1)),
                        s: self.s,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.m.1 > *threshold {
                    Some(Partition {
                        m: ((*threshold).max(self.m.0), self.m.1),
                        s: self.s,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::MConditionalBiggerThan(threshold, _) => {
                let hit_by_rule = if self.m.1 > *threshold + 1 {
                    Some(Partition {
                        m: ((*threshold + 1).max(self.m.0), self.m.1),
                        s: self.s,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.m.0 < *threshold + 1 {
                    Some(Partition {
                        m: (self.m.0, (*threshold + 1).min(self.m.1)),
                        s: self.s,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::AConditionalSmallerThan(threshold, _) => {
                let hit_by_rule = if self.a.0 < *threshold {
                    Some(Partition {
                        a: (self.a.0, (*threshold).min(self.a.1)),
                        m: self.m,
                        s: self.s,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.a.1 > *threshold {
                    Some(Partition {
                        a: ((*threshold).max(self.a.0), self.a.1),
                        m: self.m,
                        s: self.s,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::AConditionalBiggerThan(threshold, _) => {
                let hit_by_rule = if self.a.1 > *threshold + 1 {
                    Some(Partition {
                        a: ((*threshold + 1).max(self.a.0), self.a.1),
                        m: self.m,
                        s: self.s,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.a.0 < *threshold + 1 {
                    Some(Partition {
                        a: (self.a.0, (*threshold + 1).min(self.a.1)),
                        m: self.m,
                        s: self.s,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::SConditionalSmallerThan(threshold, _) => {
                let hit_by_rule = if self.s.0 < *threshold {
                    Some(Partition {
                        s: (self.s.0, (*threshold).min(self.s.1)),
                        m: self.m,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.s.1 > *threshold {
                    Some(Partition {
                        s: ((*threshold).max(self.s.0), self.s.1),
                        m: self.m,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
            Rule::SConditionalBiggerThan(threshold, _) => {
                let hit_by_rule = if self.s.1 > *threshold + 1 {
                    Some(Partition {
                        s: ((*threshold + 1).max(self.s.0), self.s.1),
                        m: self.m,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                let not_hit = if self.s.0 < *threshold + 1 {
                    Some(Partition {
                        s: (self.s.0, (*threshold + 1).min(self.s.1)),
                        m: self.m,
                        a: self.a,
                        x: self.x,
                    })
                } else {
                    None
                };
                (hit_by_rule, not_hit)
            }
        }
    }

    fn combination_count(&self) -> u64 {
        (self.x.1 - self.x.0) as u64
            * (self.m.1 - self.m.0) as u64
            * (self.a.1 - self.a.0) as u64
            * (self.s.1 - self.s.0) as u64
    }
}

fn parse_input(s: &str) -> (WorkflowList, Vec<MachinePart>) {
    let (wfl_str, mpl_str) = s.trim().split_once("\n\n").unwrap();

    (
        wfl_str.into(),
        mpl_str.lines().map(MachinePart::from).collect(),
    )
}

pub fn part_one(_input: &str) -> Option<usize> {
    let (wfl, mpl) = parse_input(_input);

    let result = mpl
        .iter()
        .filter(|mp| wfl.is_machine_part_valid(mp))
        .map(|mp| mp.x + mp.m + mp.a + mp.s)
        .sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let (wfl, _) = parse_input(_input);

    Some(
        wfl.create_partition_list()
            .iter()
            .filter(|(wfn, _)| *wfn == "A")
            .map(|(_, p)| p.combination_count())
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partitioning() {
        let wf = Workflow::from("s<1351:px,qqz");
        assert_eq!(
            wf.create_partition_list(Partition::default()),
            vec![
                (
                    "px".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1, 1351)
                    }
                ),
                (
                    "qqz".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1351, 4001)
                    }
                )
            ]
        );
        let wf = Workflow::from("s>1351:px,qqz");
        assert_eq!(
            wf.create_partition_list(Partition::default()),
            vec![
                (
                    "px".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1352, 4001)
                    }
                ),
                (
                    "qqz".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1, 1352)
                    }
                )
            ]
        );
        let wf = Workflow::from("s>1351:px,s>2351:px,qqz");
        assert_eq!(
            wf.create_partition_list(Partition::default()),
            vec![
                (
                    "px".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1352, 4001)
                    }
                ),
                (
                    "qqz".to_string(),
                    Partition {
                        x: (1, 4001),
                        m: (1, 4001),
                        a: (1, 4001),
                        s: (1, 1352)
                    }
                )
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(19114));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(167409079868000));
    }
}
