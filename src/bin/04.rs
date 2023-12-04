#[derive(Debug)]
pub struct Scratchcard {
    _id: usize,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl Scratchcard {
    fn find_winning_numbers_in_your_numbers(&self) -> Vec<u32> {
        self.winning_numbers
            .iter()
            .filter(|wn| self.your_numbers.contains(wn))
            .cloned()
            .collect()
    }

    fn winning_score(&self) -> u32 {
        let wn = self.find_winning_numbers_in_your_numbers().len() as u32;
        if wn == 0 {
            return 0;
        }
        (2 as u32).pow(wn - 1)
    }
}

impl From<(u32, (Vec<u32>, Vec<u32>))> for Scratchcard {
    fn from(value: (u32, (Vec<u32>, Vec<u32>))) -> Self {
        Self {
            _id: value.0 as usize,
            winning_numbers: value.1 .0,
            your_numbers: value.1 .1,
        }
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, space1, u32},
        combinator::into,
        error::Error as NomError,
        multi::separated_list1,
        sequence::{delimited, pair, preceded, separated_pair},
        Finish, IResult,
    };

    use crate::Scratchcard;

    pub fn parse_scratchcards(s: &str) -> Result<Vec<Scratchcard>, NomError<&str>> {
        let (_, x) = separated_list1(line_ending, parse_scratchcard)(s).finish()?;
        Ok(x)
    }

    fn parse_scratchcard(s: &str) -> IResult<&str, Scratchcard> {
        into(parse_scratchcard_raw)(s)
    }

    fn parse_scratchcard_raw(s: &str) -> IResult<&str, (u32, (Vec<u32>, Vec<u32>))> {
        separated_pair(parse_id, pair(tag(":"), space1), parse_card_content)(s)
    }

    fn parse_id(s: &str) -> IResult<&str, u32> {
        preceded(pair(tag("Card"), space1), u32)(s)
    }

    fn parse_card_content(s: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
        separated_pair(
            parse_number_list,
            delimited(space1, tag("|"), space1),
            parse_number_list,
        )(s)
    }

    fn parse_number_list(s: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(space1, u32)(s)
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let cards = parser::parse_scratchcards(_input).unwrap();
    Some(cards.iter().map(|c| c.winning_score()).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    let cards = parser::parse_scratchcards(_input).unwrap();

    let mut card_count = vec![1 as u32; cards.len()];
    for (idx, card) in cards.iter().enumerate() {
        let ws = card.find_winning_numbers_in_your_numbers().len();
        for i in (idx + 1)..(idx + 1 + ws) {
            card_count[i] += card_count[idx];
        }
    }

    Some(card_count.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(30));
    }
}
