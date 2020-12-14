use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
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

#[aoc_generator(day5)]
fn read_seat_ids(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input
        .lines()
        .map(|line| line.parse::<Seat>())
        .map_results(|seat| seat.seat_id())
        .collect()
}

#[aoc(day5, part1)]
fn part1(seat_ids: &[u32]) -> Option<u32> {
    seat_ids.iter().max().cloned()
}

#[aoc(day5, part2)]
fn part2(seat_ids: &[u32]) -> Option<u32> {
    seat_ids
        .iter()
        .sorted()
        .collect_vec()
        .windows(2)
        .find_map(|pair| {
            if pair[0] + 1 == pair[1] - 1 {
                Some(pair[0] + 1)
            } else {
                None
            }
        })
}

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

    const INPUT: &str = include_str!("../input/2020/day5.txt");

    #[test]
    fn decodes_examples() {
        let seats: Result<Vec<Seat>, ParseIntError> = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
            .iter()
            .map(|code| code.parse())
            .collect();

        assert_eq!(seats, Ok(EXAMPLE_SEATS.to_vec()));
    }

    #[test]
    fn calculates_seat_id() {
        let ids: Vec<u32> = EXAMPLE_SEATS.iter().map(|seat| seat.seat_id()).collect();
        assert_eq!(ids, &[567, 119, 820]);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            read_seat_ids(INPUT).map(|seat_ids| part1(&seat_ids)),
            Ok(Some(838))
        );
    }
    #[test]
    fn solves_part2() {
        assert_eq!(
            read_seat_ids(INPUT).map(|seat_ids| part2(&seat_ids)),
            Ok(Some(714))
        );
    }
}
