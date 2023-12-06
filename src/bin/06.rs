// t is the time the button is held
// m is the maximum race duration
// y is the distance the boat goes
// distance = speed   * time
// y(t) = d = (m - t) * t
// so we want to solve for t
// 0 = - t^2 + m * t - d
// t = (-m +- sqrt(m^2 - 4d)) / -2

use itertools::Itertools;

pub fn parse_input(s: &str) -> Vec<(u64, u64)> {
    let (time_str, distance_str) = s.trim().split_once('\n').unwrap();
    let times = time_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();
    let distances = distance_str
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect_vec();

    times.into_iter().zip(distances).collect_vec()
}

pub fn parse_input2(s: &str) -> (u64, u64) {
    let (time_str, distance_str) = s.trim().split_once('\n').unwrap();
    let time = time_str
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = distance_str
        .split_once(':')
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    (time, distance)
}

pub fn solve_race(time: u64, distance: u64) -> (u64, u64) {
    let root = ((time.pow(2) - 4 * distance) as f64).sqrt();
    (
        ((time as f64 - root) / 2.0) as u64 + 1,
        ((time as f64 + root) / 2.0).ceil() as u64,
    )
}

pub fn part_one(_input: &str) -> Option<u64> {
    let races = parse_input(_input);

    let result: u64 = races
        .iter()
        .map(|r| solve_race(r.0, r.1))
        .map(|(s, e)| e - s)
        .product();
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let race = parse_input2(_input);

    let result = solve_race(race.0, race.1);
    Some(result.1 - result.0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(288));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(71503));
    }
}
