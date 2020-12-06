use advent_of_code::read_from_file;
use anyhow::anyhow;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;

fn main() {
    let maximum_seat_id = read_from_file("inputs/day05.txt")
        .map(|seat: Seat| seat.seat_id())
        .max()
        .expect("No input?");
    println!("Maximum seat id: {}", maximum_seat_id);
}

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
    type Err = anyhow::Error;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(?P<row>[BF]{7})(?P<column>[LR]{3})").unwrap();
        }
        let captures = RE
            .captures(code)
            .ok_or_else(|| anyhow!("Invalid code: {}", code))?;
        let row = decode_binary('F', 'B', from_named(&captures, "row")?)?;
        let column = decode_binary('L', 'R', from_named(&captures, "column")?)?;
        Ok(Seat { row, column })
    }
}

fn from_named<'a, 'b>(cap: &'a Captures, name: &'b str) -> Result<&'a str> {
    cap.name(name)
        .map(|name| name.as_str())
        .ok_or_else(|| anyhow!("Missing '{}' in {}", name, cap.get(0).unwrap().as_str()))
}

fn decode_binary(zero: char, one: char, binary_code: &str) -> Result<u32> {
    u32::from_str_radix(&binary_code.replace(zero, "0").replace(one, "1"), 2)
        .with_context(|| format!("Invalid binary code: {}", binary_code))
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

    #[test]
    fn decodes_examples() -> Result<()> {
        let seats: Vec<Seat> = ["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
            .iter()
            .map(|code| code.parse())
            .collect::<Result<_>>()?;

        assert_eq!(seats, EXAMPLE_SEATS);
        Ok(())
    }

    #[test]
    fn calculates_seat_id() {
        let ids: Vec<u32> = EXAMPLE_SEATS.iter().map(|seat| seat.seat_id()).collect();
        assert_eq!(ids, &[567, 119, 820]);
    }
}
