use advent_of_code::algebra_helpers::Point3;

#[derive(Debug)]
pub struct CubeSet(Point3<u32>);

impl CubeSet {
    fn is_valid_part1(&self) -> bool {
        self.0[0] < 12 && self.0[1] < 13 && self.0[2] < 14
    }

    fn power(&self) -> u32 {
        self.0[0] * self.0[1] * self.0[2]
    }
}

impl From<Vec<(u32, &str)>> for CubeSet {
    fn from(values: Vec<(u32, &str)>) -> Self {
        let mut res: Point3<u32> = Point3::zero();

        for (amount, color) in values {
            res = match color {
                "red" => res + Point3::new(amount, 0, 0),
                "green" => res + Point3::new(0, amount, 0),
                "blue" => res + Point3::new(0, 0, amount),
                _ => res,
            }
        }

        Self(res)
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    pulls: Vec<CubeSet>,
}

impl Game {
    fn is_valid_part1(&self) -> bool {
        self.pulls.iter().all(|cs| cs.is_valid_part1())
    }

    fn get_minimum_cubes(&self) -> CubeSet {
        let mut min_cubes: Point3<u32> = Point3::zero();

        for p in self.pulls.iter() {
            if p.0[0] > min_cubes.0[0] {
                min_cubes.0[0] = p.0[0];
            }
            if p.0[1] > min_cubes.0[1] {
                min_cubes.0[1] = p.0[1];
            }
            if p.0[2] > min_cubes.0[2] {
                min_cubes.0[2] = p.0[2];
            }
        }

        CubeSet(min_cubes)
    }
}

impl From<(u32, Vec<CubeSet>)> for Game {
    fn from(value: (u32, Vec<CubeSet>)) -> Self {
        Self {
            id: value.0 as usize,
            pulls: value.1,
        }
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, line_ending, space1, u32},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        Finish, IResult,
    };

    use crate::{CubeSet, Game};

    pub fn parse_games(s: &str) -> Result<Vec<Game>, NomError<&str>> {
        let (_, x) = separated_list1(line_ending, parse_game)(s).finish()?;
        Ok(x)
    }

    fn parse_game(s: &str) -> IResult<&str, Game> {
        into(parse_game_raw)(s)
    }

    fn parse_game_raw(s: &str) -> IResult<&str, (u32, Vec<CubeSet>)> {
        separated_pair(parse_id, tag(": "), parse_cubeset_list)(s)
    }

    fn parse_id(s: &str) -> IResult<&str, u32> {
        preceded(tag("Game "), u32)(s)
    }

    fn parse_cubeset_list(s: &str) -> IResult<&str, Vec<CubeSet>> {
        separated_list1(tag("; "), parse_cubeset)(s)
    }

    fn parse_cubeset(s: &str) -> IResult<&str, CubeSet> {
        into(parse_cubeset_raw)(s)
    }

    fn parse_cubeset_raw(s: &str) -> IResult<&str, Vec<(u32, &str)>> {
        separated_list1(tag(", "), parse_cubecolor)(s)
    }

    fn parse_cubecolor(s: &str) -> IResult<&str, (u32, &str)> {
        separated_pair(u32, space1, alpha1)(s)
    }
}

pub fn part_one(_input: &str) -> Option<usize> {
    let games = parser::parse_games(_input).unwrap();
    Some(
        games
            .iter()
            .filter(|g| g.is_valid_part1())
            .map(|g| g.id)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    let games = parser::parse_games(_input).unwrap();
    Some(games.iter().map(|g| g.get_minimum_cubes().power()).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(2286));
    }
}
