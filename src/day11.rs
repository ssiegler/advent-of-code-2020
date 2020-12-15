use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::ParseError;
use std::time::Instant;

#[derive(Debug, PartialEq, Clone)]
struct Seats {
    columns: usize,
    tiles: Vec<u8>,
}

impl Seats {
    fn next_round(&mut self) {
        let mut tiles = Vec::with_capacity(self.tiles.len());

        for (row_index, row) in self.rows().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                tiles.push(
                    match (cell, self.count_occupied_neighbors(row_index, column_index)) {
                        (b'L', 0) => b'#',
                        (b'#', n) if (n >= 4) => b'L',
                        _ => *cell,
                    },
                );
            }
        }
        self.tiles = tiles;
    }

    fn iterate_until_stable(&mut self) {
        let mut current = self.tiles.clone();
        loop {
            self.next_round();
            if current == self.tiles {
                return;
            }
            current = self.tiles.clone();
        }
    }

    fn count_occupied_neighbors(&self, row: usize, column: usize) -> usize {
        self.rows()
            .enumerate()
            .skip_while(|(index, _)| *index + 1 < row)
            .take_while(|(index, _)| *index <= row + 1)
            .flat_map(|(row_index, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .skip_while(|(index, _)| *index + 1 < column)
                    .take_while(|(index, _)| *index <= column + 1)
                    .filter(move |(column_index, _)| *column_index != column || row_index != row)
                    .map(|(_, cell)| *cell)
            })
            .filter(|cell| *cell == b'#')
            .count()
    }

    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.tiles.chunks(self.columns)
    }

    fn count_occupied(&self) -> usize {
        // bytecount::count(&self.tiles, b'#')
        self.tiles.iter().filter(|cell| **cell == b'#').count()
    }
}

#[aoc_generator(day11)]
fn read_seats(input: &str) -> Result<Seats, ParseError> {
    Seats::from_str(input)
}

#[aoc(day11, part1)]
fn count_stabilized_seats(seats: &Seats) -> usize {
    let mut seats = seats.clone();
    seats.iterate_until_stable();
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
        Ok(Seats {
            columns: columns.unwrap_or(0),
            tiles,
        })
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self
            .rows()
            .map(String::from_utf8_lossy)
            .intersperse("\n".into())
        {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use lazy_static::lazy_static;

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
        seats.next_round();
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
            .count_occupied_neighbors(0, 2),
            4
        );
    }

    #[test]
    fn correctly_computes_second_round() {
        let mut seats = SEATS.clone();
        seats.next_round();
        seats.next_round();
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
