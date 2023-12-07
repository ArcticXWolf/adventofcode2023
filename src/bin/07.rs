#![feature(iter_order_by)]
use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl Card {
    fn cmp_part2(&self, other: &Self) -> Ordering {
        let self_int = match self {
            Self::Jack => 0,
            _ => *self as u32,
        };
        let other_int = match other {
            Self::Jack => 0,
            _ => *other as u32,
        };
        self_int.cmp(&other_int)
    }
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(format!("Could not match card {:?}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new_from_cards_part1(counts: &HashMap<Card, usize>) -> Self {
        if counts.values().any(|c| *c == 5) {
            return Self::FiveOfAKind;
        } else if counts.values().any(|c| *c == 4) {
            return Self::FourOfAKind;
        } else if counts.values().all(|c| *c == 3 || *c == 2) {
            return Self::FullHouse;
        } else if counts.values().any(|c| *c == 3) {
            return Self::ThreeOfAKind;
        } else if counts.values().filter(|c| **c == 2).count() == 2 {
            return Self::TwoPair;
        } else if counts.values().any(|c| *c == 2) {
            return Self::Pair;
        } else {
            return Self::HighCard;
        }
    }

    fn new_from_cards_part2(counts: &HashMap<Card, usize>) -> Self {
        let wildcards = match counts.get(&Card::Jack) {
            Some(x) => *x,
            None => 0,
        };
        let counts_without_wildcards = counts
            .iter()
            .filter(|(c, _)| **c != Card::Jack)
            .map(|(_, u)| *u)
            .collect_vec();

        if counts_without_wildcards.iter().any(|c| *c + wildcards == 5) || wildcards == 5 {
            return Self::FiveOfAKind;
        } else if counts_without_wildcards.iter().any(|c| c + wildcards == 4) {
            return Self::FourOfAKind;
        } else if Self::is_full_house(&counts_without_wildcards, wildcards) {
            return Self::FullHouse;
        } else if counts_without_wildcards.iter().any(|c| *c + wildcards == 3) {
            return Self::ThreeOfAKind;
        } else if counts_without_wildcards.iter().filter(|c| **c == 2).count() == 2 {
            return Self::TwoPair;
        } else if counts_without_wildcards.iter().any(|c| *c + wildcards == 2) {
            return Self::Pair;
        } else {
            return Self::HighCard;
        }
    }

    fn is_full_house(counts: &[usize], wildcards: usize) -> bool {
        (wildcards == 0 && counts.iter().all(|c| *c == 3 || *c == 2))
            || (wildcards == 1 && counts.iter().all(|c| *c == 2))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl Hand {
    fn new(s: &str, part1: bool) -> Result<Self, String> {
        if s.chars().count() != 5 {
            return Err(format!("Hand string {:?} is not length 5", s));
        }

        let cards: Result<Vec<Card>, String> = s.chars().map(|c| c.try_into()).collect();

        if let Ok(c) = cards {
            let counts = c.clone().into_iter().counts();
            let hand_type = if part1 {
                HandType::new_from_cards_part1(&counts)
            } else {
                HandType::new_from_cards_part2(&counts)
            };
            Ok(Self {
                cards: c,
                hand_type,
            })
        } else {
            Err(cards.unwrap_err())
        }
    }

    fn cmp_part2(&self, other: &Self) -> Ordering {
        let hand_typ_cmp = self.hand_type.cmp(&other.hand_type);
        match hand_typ_cmp {
            Ordering::Equal => self
                .cards
                .iter()
                .cmp_by(other.cards.iter(), Card::cmp_part2),
            _ => hand_typ_cmp,
        }
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let mut hand_bid_pairs: Vec<(Hand, u32)> = _input
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(hand_s, bid_s)| {
            (
                Hand::new(hand_s, true).unwrap(),
                bid_s.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();

    hand_bid_pairs.sort();

    Some(
        hand_bid_pairs
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) as u32 * *bid)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut hand_bid_pairs: Vec<(Hand, u32)> = _input
        .lines()
        .filter_map(|l| l.split_once(" "))
        .map(|(hand_s, bid_s)| {
            (
                Hand::new(hand_s, false).unwrap(),
                bid_s.parse::<u32>().unwrap(),
            )
        })
        .collect_vec();

    hand_bid_pairs.sort_by(|a, b| a.0.cmp_part2(&b.0));

    Some(
        hand_bid_pairs
            .iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) as u32 * *bid)
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(6440));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(6839));
    }

    #[test]
    fn test_hands() {
        assert_eq!(
            Hand::new("AAAAA", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
                hand_type: HandType::FiveOfAKind
            })
        );
        assert_eq!(
            Hand::new("AAAAJ", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Jack],
                hand_type: HandType::FourOfAKind
            })
        );
        assert_eq!(
            Hand::new("AAAJJ", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Jack, Card::Jack],
                hand_type: HandType::FullHouse
            })
        );
        assert_eq!(
            Hand::new("AAAJT", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Jack, Card::Ten],
                hand_type: HandType::ThreeOfAKind
            })
        );
        assert_eq!(
            Hand::new("AAJJT", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Jack, Card::Jack, Card::Ten],
                hand_type: HandType::TwoPair
            })
        );
        assert_eq!(
            Hand::new("AAJ9T", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Jack, Card::Nine, Card::Ten],
                hand_type: HandType::Pair
            })
        );
        assert_eq!(
            Hand::new("AKQJT", true),
            Ok(Hand {
                cards: vec![Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Ten],
                hand_type: HandType::HighCard
            })
        );

        assert_eq!(
            Hand::new("AAAJJ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Jack, Card::Jack],
                hand_type: HandType::FiveOfAKind
            })
        );
        assert_eq!(
            Hand::new("AAQJJ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Queen, Card::Jack, Card::Jack],
                hand_type: HandType::FourOfAKind
            })
        );
        assert_eq!(
            Hand::new("AAAQQ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Ace, Card::Queen, Card::Queen],
                hand_type: HandType::FullHouse
            })
        );
        assert_eq!(
            Hand::new("AAJQQ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Ace, Card::Jack, Card::Queen, Card::Queen],
                hand_type: HandType::FullHouse
            })
        );
        assert_eq!(
            Hand::new("AJJQQ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::Jack, Card::Jack, Card::Queen, Card::Queen],
                hand_type: HandType::FourOfAKind
            })
        );
        assert_eq!(
            Hand::new("AKQJJ", false),
            Ok(Hand {
                cards: vec![Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Jack],
                hand_type: HandType::ThreeOfAKind
            })
        );
    }

    #[test]
    fn test_hand_orderings() {
        assert!(Hand::new("AAAAA", true) == Hand::new("AAAAA", true));
        assert!(Hand::new("AAAAA", true) > Hand::new("AAAAJ", true));
        assert!(Hand::new("AAAAJ", true) > Hand::new("AAAAT", true));
        assert!(Hand::new("AAAJJ", true) < Hand::new("AAAKK", true));
        assert!(Hand::new("23456", true) < Hand::new("65432", true));
    }
}
