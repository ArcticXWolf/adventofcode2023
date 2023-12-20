use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_of_code::helpers::lcm_mn;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high"),
            Self::Low => write!(f, "low"),
        }
    }
}

trait Module {
    fn get_name(&self) -> String;
    fn get_targets(&self) -> Vec<String>;
    fn add_other_modules_as_inputs(&mut self, name: &str, targets: Vec<String>);
    fn process_pulse(&mut self, source: &str, input: Pulse) -> Vec<(String, String, Pulse)>;
}

struct BroadcasterModule {
    name: String,
    targets: Vec<String>,
}

impl From<&str> for BroadcasterModule {
    fn from(value: &str) -> Self {
        let (name, target_str) = value.trim().split_once(" -> ").unwrap();
        let targets = target_str.split(", ").map(|s| s.to_string()).collect_vec();
        Self {
            name: name.to_string(),
            targets,
        }
    }
}

impl Module for BroadcasterModule {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_targets(&self) -> Vec<String> {
        self.targets.clone()
    }
    fn add_other_modules_as_inputs(&mut self, _name: &str, _targets: Vec<String>) {}
    fn process_pulse(&mut self, _source: &str, input: Pulse) -> Vec<(String, String, Pulse)> {
        let mut result = vec![];
        for destination in self.targets.iter() {
            result.push((self.name.clone(), destination.clone(), input));
        }
        result
    }
}

struct FlipFlopModule {
    name: String,
    targets: Vec<String>,
    state: bool,
}

impl From<&str> for FlipFlopModule {
    fn from(value: &str) -> Self {
        let (name, target_str) = value.trim().split_once(" -> ").unwrap();
        let targets = target_str.split(", ").map(|s| s.to_string()).collect_vec();
        Self {
            name: name.to_string(),
            targets,
            state: false,
        }
    }
}

impl Module for FlipFlopModule {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_targets(&self) -> Vec<String> {
        self.targets.clone()
    }
    fn add_other_modules_as_inputs(&mut self, _name: &str, _targets: Vec<String>) {}
    fn process_pulse(&mut self, _source: &str, input: Pulse) -> Vec<(String, String, Pulse)> {
        match input {
            Pulse::High => vec![],
            Pulse::Low => {
                self.state = !self.state;
                let output = if self.state { Pulse::High } else { Pulse::Low };

                let mut result = vec![];
                for destination in self.targets.iter() {
                    result.push((self.name.clone(), destination.clone(), output));
                }
                result
            }
        }
    }
}

struct ConjunctionModule {
    name: String,
    targets: Vec<String>,
    state: HashMap<String, Pulse>,
}

impl From<&str> for ConjunctionModule {
    fn from(value: &str) -> Self {
        let (name, target_str) = value.trim().split_once(" -> ").unwrap();
        let targets = target_str.split(", ").map(|s| s.to_string()).collect_vec();
        Self {
            name: name.to_string(),
            targets,
            state: HashMap::default(),
        }
    }
}

impl Module for ConjunctionModule {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_targets(&self) -> Vec<String> {
        self.targets.clone()
    }
    fn add_other_modules_as_inputs(&mut self, name: &str, targets: Vec<String>) {
        if targets.contains(&self.name) {
            self.state.insert(name.to_string(), Pulse::Low);
        }
    }

    fn process_pulse(&mut self, source: &str, input: Pulse) -> Vec<(String, String, Pulse)> {
        assert!(self.state.contains_key(source));
        self.state.insert(source.to_string(), input);

        let output = if self.state.values().all(|&p| p == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        let mut result = vec![];
        for destination in self.targets.iter() {
            result.push((self.name.clone(), destination.clone(), output));
        }
        result
    }
}

struct Bus {
    modules: HashMap<String, Box<dyn Module>>,
}

impl From<&str> for Bus {
    fn from(value: &str) -> Self {
        let mut modules = HashMap::default();
        let mut mod_name_target_list = vec![];

        for module_str in value.trim().lines() {
            let module: Box<dyn Module> = match module_str.chars().nth(0) {
                Some('%') => Box::new(FlipFlopModule::from(&module_str[1..])),
                Some('&') => Box::new(ConjunctionModule::from(&module_str[1..])),
                Some('b') => Box::new(BroadcasterModule::from(module_str)),
                _ => unimplemented!(),
            };
            let name = module.get_name();
            mod_name_target_list.push((name.clone(), module.get_targets()));
            modules.insert(name.clone(), module);
        }
        for (_, m1) in modules.iter_mut() {
            for (name, targets) in &mod_name_target_list {
                m1.add_other_modules_as_inputs(name.as_str(), targets.clone());
            }
        }

        Self { modules }
    }
}

impl Bus {
    fn trigger_one_pulse(&mut self, node_to_watch: &str) -> (usize, usize, Vec<String>) {
        let mut pulses = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
        let mut idx = 0;
        let mut count_high = 0;
        let mut count_low = 1;
        let mut watcher = HashSet::new();

        while idx < pulses.len() {
            if let Some((source_name, destination_name, pulse)) = pulses.get(idx) {
                if destination_name == node_to_watch && *pulse == Pulse::High {
                    watcher.insert(source_name.clone());
                }
                let next_pulses = self.trigger_pulse(source_name, destination_name, *pulse);
                count_high += next_pulses.iter().filter(|p| p.2 == Pulse::High).count();
                count_low += next_pulses.iter().filter(|p| p.2 == Pulse::Low).count();
                pulses.extend(next_pulses);
            }
            idx += 1;
        }
        (count_high, count_low, watcher.into_iter().collect_vec())
    }

    fn trigger_pulse(
        &mut self,
        source_name: &str,
        destination_name: &str,
        pulse: Pulse,
    ) -> Vec<(String, String, Pulse)> {
        if let Some(module) = self.modules.get_mut(destination_name) {
            let r = module.process_pulse(source_name, pulse);
            return r
                .into_iter()
                .map(|(s, d, p)| (s.to_string(), d.to_string(), p))
                .collect_vec();
        }
        vec![]
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let mut bus = Bus::from(_input);
    let mut count_high = 0;
    let mut count_low = 0;
    for _ in 0..1000 {
        let (ch, cl, _) = bus.trigger_one_pulse("zh");
        count_high += ch;
        count_low += cl;
    }

    Some(count_high * count_low)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut bus = Bus::from(_input);
    let mut map = HashMap::new();
    let mut turn: usize = 0;

    while map.is_empty() || map.values().any(|c: &Vec<usize>| c.len() < 2) {
        let (_, _, nodes) = bus.trigger_one_pulse("zh");

        for node in nodes {
            if let Some(entry) = map.get_mut(&node) {
                entry.push(turn);
            } else {
                map.insert(node, vec![turn]);
            }
        }
        turn += 1;
    }

    let cycles = map
        .values()
        .map(|c| c.iter().nth(1).unwrap() - c.iter().nth(0).unwrap())
        .collect_vec();

    Some(lcm_mn(&cycles))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_module() {
        let mut broadcast = BroadcasterModule::from("broadcaster -> ls, bv");
        assert_eq!(broadcast.name, "broadcaster");
        assert_eq!(broadcast.targets, vec!["ls", "bv"]);
        assert_eq!(
            broadcast.process_pulse("pz", Pulse::Low),
            vec![
                ("broadcaster".to_string(), "ls".to_string(), Pulse::Low),
                ("broadcaster".to_string(), "bv".to_string(), Pulse::Low)
            ]
        );
        assert_eq!(
            broadcast.process_pulse("pz", Pulse::High),
            vec![
                ("broadcaster".to_string(), "ls".to_string(), Pulse::High),
                ("broadcaster".to_string(), "bv".to_string(), Pulse::High)
            ]
        );
    }

    #[test]
    fn test_flipflop_module() {
        let mut flipflop = FlipFlopModule::from("ff -> ls, bv");
        assert_eq!(flipflop.name, "ff");
        assert_eq!(flipflop.targets, vec!["ls", "bv"]);
        assert_eq!(flipflop.state, false);
        assert_eq!(flipflop.process_pulse("pz", Pulse::High), vec![]);
        assert_eq!(flipflop.state, false);
        assert_eq!(
            flipflop.process_pulse("pz", Pulse::Low),
            vec![
                ("ff".to_string(), "ls".to_string(), Pulse::High),
                ("ff".to_string(), "bv".to_string(), Pulse::High)
            ]
        );
        assert_eq!(flipflop.state, true);
        assert_eq!(flipflop.process_pulse("pz", Pulse::High), vec![]);
        assert_eq!(flipflop.state, true);
        assert_eq!(
            flipflop.process_pulse("pz", Pulse::Low),
            vec![
                ("ff".to_string(), "ls".to_string(), Pulse::Low),
                ("ff".to_string(), "bv".to_string(), Pulse::Low)
            ]
        );
        assert_eq!(flipflop.state, false);
    }

    #[test]
    fn test_conjunction_module() {
        let mut inverter = ConjunctionModule::from("inv -> ls");
        assert_eq!(inverter.name, "inv");
        assert_eq!(inverter.targets, vec!["ls"]);
        inverter.add_other_modules_as_inputs("pz", vec!["inv".to_string()]);
        assert_eq!(
            inverter.process_pulse("pz", Pulse::High),
            vec![("inv".to_string(), "ls".to_string(), Pulse::Low)]
        );
        assert_eq!(
            inverter.process_pulse("pz", Pulse::Low),
            vec![("inv".to_string(), "ls".to_string(), Pulse::High)]
        );
        let mut conjunction = ConjunctionModule::from("con -> ls");
        assert_eq!(conjunction.name, "con");
        assert_eq!(conjunction.targets, vec!["ls"]);
        conjunction.add_other_modules_as_inputs("pz", vec!["con".to_string()]);
        conjunction.add_other_modules_as_inputs("ff", vec!["con".to_string()]);
        assert_eq!(
            conjunction.process_pulse("pz", Pulse::High),
            vec![("con".to_string(), "ls".to_string(), Pulse::High)]
        );
        assert_eq!(
            conjunction.process_pulse("ff", Pulse::Low),
            vec![("con".to_string(), "ls".to_string(), Pulse::High)]
        );
        assert_eq!(
            conjunction.process_pulse("ff", Pulse::High),
            vec![("con".to_string(), "ls".to_string(), Pulse::Low)]
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(45000000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1));
    }
}
