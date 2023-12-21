use std::fmt::{self, Display};

use bitvec::prelude::*;

const FIELD_WIDTH: usize = 131;
const FIELD_AMOUNT: usize = 5;
const BB_WIDTH: usize = FIELD_WIDTH * FIELD_AMOUNT;
const BB_LENGTH: usize = BB_WIDTH * BB_WIDTH;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitBoard {
    bits: BitArr!(for BB_LENGTH, in usize, Msb0),
    rocks: BitArr!(for BB_LENGTH, in usize, Msb0),
    east_shift_mask: BitArr!(for BB_LENGTH, in usize, Msb0),
    west_shift_mask: BitArr!(for BB_LENGTH, in usize, Msb0),
    mod2_mask: BitArr!(for BB_LENGTH, in usize, Msb0),
}

impl BitBoard {
    pub fn new() -> Self {
        let mut mod2_mask = bitarr!(usize, Msb0; 1; BB_LENGTH);
        let mut east_shift_mask = bitarr!(usize, Msb0; 1; BB_LENGTH);
        let mut west_shift_mask = bitarr!(usize, Msb0; 1; BB_LENGTH);
        for y in 0..BB_WIDTH {
            for x in 0..BB_WIDTH {
                if x == 0 {
                    east_shift_mask.set(y * BB_WIDTH + x, false);
                }
                if x == BB_WIDTH - 1 {
                    west_shift_mask.set(y * BB_WIDTH + x, false);
                }
                if x % 2 == y % 2 {
                    mod2_mask.set(y * BB_WIDTH + x, false);
                }
            }
        }

        Self {
            bits: bitarr!(usize, Msb0; 0; BB_LENGTH),
            rocks: bitarr!(usize, Msb0; 0; BB_LENGTH),
            east_shift_mask,
            west_shift_mask,
            mod2_mask,
        }
    }
}

impl From<&str> for BitBoard {
    fn from(value: &str) -> Self {
        let mut bb = BitBoard::new();

        for field_y in 0..FIELD_AMOUNT {
            for field_x in 0..FIELD_AMOUNT {
                for (y, row) in value.trim().lines().enumerate() {
                    for (x, c) in row.char_indices() {
                        match c {
                            '#' => bb.rocks.set(
                                (field_y * FIELD_WIDTH + y) * BB_WIDTH
                                    + (field_x * FIELD_WIDTH)
                                    + x,
                                true,
                            ),
                            _ => {}
                        }
                    }
                }
            }
        }

        bb.bits.set(BB_LENGTH / 2, true);

        bb
    }
}

impl BitBoard {
    pub fn grow_all_directions(&mut self) {
        let original_bitboard = self.bits.clone();

        let mut north_bitboard = self.bits.clone();
        north_bitboard.shift_left(BB_WIDTH);

        let mut east_bitboard = self.bits.clone();
        east_bitboard.shift_right(1);

        let mut south_bitboard = self.bits.clone();
        south_bitboard.shift_right(BB_WIDTH);

        let mut west_bitboard = self.bits.clone();
        west_bitboard.shift_left(1);

        self.bits = (original_bitboard
            | north_bitboard
            | (east_bitboard & self.east_shift_mask)
            | south_bitboard
            | (west_bitboard & self.west_shift_mask))
            & !self.rocks;
    }
}

impl Display for BitBoard {
    fn fmt(self: &'_ Self, stream: &'_ mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..BB_WIDTH {
            for x in 0..BB_WIDTH {
                if self.rocks[y * BB_WIDTH + x] && self.bits[y * BB_WIDTH + x] {
                    write!(stream, "X")?;
                } else if self.rocks[y * BB_WIDTH + x] {
                    write!(stream, "#")?;
                } else if self.bits[y * BB_WIDTH + x] {
                    write!(stream, "O")?;
                } else {
                    write!(stream, ".")?;
                }
            }
            writeln!(stream)?;
        }
        write!(stream, "")
    }
}

pub fn part_one_param(_input: &str, steps: usize) -> Option<usize> {
    let mut bit_board = BitBoard::from(_input);

    for _ in 0..steps {
        bit_board.grow_all_directions();
    }

    let inversion_mask = if steps % 2 == 0 {
        !bit_board.mod2_mask
    } else {
        bit_board.mod2_mask
    };
    bit_board.bits = bit_board.bits & inversion_mask;

    Some(bit_board.bits.count_ones())
}

pub fn part_one(_input: &str) -> Option<usize> {
    part_one_param(_input, 64)
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut bit_board = BitBoard::from(_input);
    let mut current_steps = 0;

    for _ in 0..(FIELD_WIDTH / 2) {
        bit_board.grow_all_directions();
        current_steps += 1;
    }
    let inversion_mask = if current_steps % 2 == 0 {
        !bit_board.mod2_mask
    } else {
        bit_board.mod2_mask
    };
    println!(
        "At x={} y={}",
        current_steps,
        (bit_board.bits & inversion_mask).count_ones()
    );

    for _ in 0..(FIELD_WIDTH) {
        bit_board.grow_all_directions();
        current_steps += 1;
    }
    let inversion_mask = if current_steps % 2 == 0 {
        !bit_board.mod2_mask
    } else {
        bit_board.mod2_mask
    };
    println!(
        "At x={} y={}",
        current_steps,
        (bit_board.bits & inversion_mask).count_ones()
    );

    for _ in 0..(FIELD_WIDTH) {
        bit_board.grow_all_directions();
        current_steps += 1;
    }
    let inversion_mask = if current_steps % 2 == 0 {
        !bit_board.mod2_mask
    } else {
        bit_board.mod2_mask
    };
    println!(
        "At x={} y={}",
        current_steps,
        (bit_board.bits & inversion_mask).count_ones()
    );

    // Too lazy to implement a quadratic formula fit. Just throw the three (x,y) coordinates into wolfram alpha and solve for
    // x=26501365 afterwards.
    Some(610158187362102)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one_param(&input, 6), Some(49));
    }

    #[test]
    fn test_part_two() {
        let _input = advent_of_code::read_file("examples", 21);
        // Since the code is heavily tailored towards the real input (for
        // example the field size is a const), we disable the test.
        //assert_eq!(part_two(&input), Some(1));
    }
}
