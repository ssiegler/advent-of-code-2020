use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use itertools::Itertools;

use crate::ParseError;

#[derive(Debug, PartialEq, Clone)]
struct Seats {
    columns: usize,
    rows: usize,
    tiles: Vec<u8>,
}

impl Seats {
    fn simulate_iteration(&mut self) {
        let mut tiles = Vec::with_capacity(self.tiles.len());

        for (index, cell) in self.tiles.iter().enumerate() {
            tiles.push(match (cell, self.count_occupied_neighbors(index)) {
                (b'L', 0) => b'#',
                (b'#', n) if (n >= 4) => b'L',
                _ => *cell,
            });
        }
        self.tiles = tiles;
    }

    fn iterate_until_stable(&mut self, iteration: fn(&mut Self)) {
        let mut current = self.tiles.clone();
        loop {
            iteration(self);
            if current == self.tiles {
                return;
            }
            current = self.tiles.clone();
        }
    }

    fn count_occupied_neighbors(&self, index: usize) -> usize {
        let row = index / self.columns;
        let column = index % self.columns;
        let mut count = 0;
        for neighbor_row in (max(1, row) - 1)..min(self.rows, row + 2) {
            for neighbor_column in (max(1, column) - 1)..min(self.columns, column + 2) {
                if neighbor_column == column && neighbor_row == row {
                    continue;
                } else if self.get(neighbor_row, neighbor_column) == Some(b'#') {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_occupied(&self) -> usize {
        bytecount::count(&self.tiles, b'#')
    }

    fn get(&self, row: usize, column: usize) -> Option<u8> {
        self.tiles.get(row * self.columns + column).cloned()
    }
}

#[aoc_generator(day11)]
fn read_seats(input: &str) -> Result<Seats, ParseError> {
    Seats::from_str(input)
}

#[aoc(day11, part1)]
fn count_stabilized_seats(seats: &Seats) -> usize {
    let mut seats = seats.clone();
    seats.iterate_until_stable(Seats::simulate_iteration);
    seats.count_occupied()
}

impl FromStr for Seats {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut columns = None;
        let mut tiles = Vec::new();
        for line in input.lines() {
            if *columns.get_or_insert(line.len()) != line.len() {
                return Err(ParseError::FormatError);
            }
            tiles.extend(line.as_bytes());
        }
        let columns = columns.unwrap_or(0);
        let rows = tiles.len() / columns;
        Ok(Seats {
            columns,
            rows,
            tiles,
        })
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for ch in self
            .tiles
            .chunks(self.columns)
            .map(|row| String::from_utf8_lossy(row))
            .intersperse("\n".into())
        {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod should {
    use lazy_static::lazy_static;

    use super::*;

    const EXAMPLE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    lazy_static! {
        static ref SEATS: Seats = Seats::from_str(EXAMPLE).expect("Failed to parse example");
    }

    #[test]
    fn parse_and_display_seats() {
        assert_eq!(SEATS.to_string(), EXAMPLE);
    }

    #[test]
    fn all_seats_become_occupied_on_first_round() {
        let mut seats = SEATS.clone();
        seats.simulate_iteration();
        assert_eq!(
            seats.to_string(),
            "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
        )
    }

    #[test]
    fn counts_correctly() {
        assert_eq!(
            Seats::from_str(
                "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            )
            .expect("Failed to read example")
            .count_occupied_neighbors(2),
            4
        );
    }

    #[test]
    fn correctly_computes_second_round() {
        let mut seats = SEATS.clone();
        seats.simulate_iteration();
        seats.simulate_iteration();
        assert_eq!(
            seats.to_string(),
            "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
        );
    }

    #[test]
    fn counts_37_occupied_seats_when_example_stabilizes() {
        assert_eq!(count_stabilized_seats(&SEATS), 37);
    }

    const INPUT: &str = include_str!("../input/2020/day11.txt");

    #[test]
    fn solves_part1() {
        assert_eq!(
            Seats::from_str(INPUT).map(|seats| count_stabilized_seats(&seats)),
            Ok(2329)
        );
    }
}
