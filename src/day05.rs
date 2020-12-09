use crate::puzzle::{Lines, Puzzle};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

type Day05 = Lines<Seat>;

#[derive(Eq, PartialEq, Debug)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let (row, column) = code.split_at(7);
        let row = decode_binary('F', 'B', row)?;
        let column = decode_binary('L', 'R', column)?;
        Ok(Seat { row, column })
    }
}

fn decode_binary(zero: char, one: char, binary_code: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(&binary_code.replace(zero, "0").replace(one, "1"), 2)
}

impl Puzzle for Day05 {
    fn solve_part1(&self) -> String {
        self.iter()
            .map(|seat| seat.seat_id())
            .max()
            .map_or_else(|| "No seats!".to_string(), |seat_id| seat_id.to_string())
    }

    fn solve_part2(&self) -> String {
        let seat_ids: Vec<u32> = self.iter().map(|seat| seat.seat_id()).sorted().collect();
        seat_ids
            .windows(2)
            .find_map(|pair| {
                if pair[0] + 1 == pair[1] - 1 {
                    Some(pair[0] + 1)
                } else {
                    None
                }
            })
            .map_or_else(
                || "Seat not found".to_string(),
                |seat_id| seat_id.to_string(),
            )
    }
}

test_puzzle!(Day05;
Example("\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL", 820, "Seat not found"),
File("inputs/day05.txt", 838, 714));

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SEATS: &[Seat] = &[
        Seat { row: 70, column: 7 },
        Seat { row: 14, column: 7 },
        Seat {
            row: 102,
            column: 4,
        },
    ];

    #[test]
    fn decodes_examples() {
        let seats: Result<Vec<Seat>, ParseIntError> = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
            .iter()
            .map(|code| code.parse())
            .collect();

        assert!(seats.is_ok());
        assert_eq!(seats.unwrap(), EXAMPLE_SEATS);
    }

    #[test]
    fn calculates_seat_id() {
        let ids: Vec<u32> = EXAMPLE_SEATS.iter().map(|seat| seat.seat_id()).collect();
        assert_eq!(ids, &[567, 119, 820]);
    }
}
