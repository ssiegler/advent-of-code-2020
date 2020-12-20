use crate::ParseError;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Notes {
    earliest_departure: usize,
    bus_ids: Vec<usize>,
}

impl FromStr for Notes {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let earliest_departure = lines.next().ok_or(ParseError::FormatError)?.parse()?;
        let bus_ids = lines
            .exactly_one()
            .map_err(|_| ParseError::FormatError)?
            .split(',')
            .filter(|item| *item != "x")
            .map(|item| item.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Notes {
            earliest_departure,
            bus_ids,
        })
    }
}

impl Notes {
    fn find_first_departure(&self) -> Option<(usize, usize)> {
        self.bus_ids
            .iter()
            .cloned()
            .map(|id| (id, next_departure(self.earliest_departure, id)))
            .min_by(|(_, departure1), (_, departure2)| departure1.cmp(departure2))
    }
}

fn next_departure(earliest: usize, bus_id: usize) -> usize {
    if earliest % bus_id == 0 {
        earliest
    } else {
        (earliest / bus_id + 1) * bus_id
    }
}

#[aoc_generator(day13, part1)]
fn read_notes(input: &str) -> Result<Notes, ParseError> {
    input.parse()
}

#[aoc(day13, part1)]
fn multiply_departure_with_wait(notes: &Notes) -> Option<usize> {
    notes
        .find_first_departure()
        .map(|(bus_id, departure)| bus_id * (departure - notes.earliest_departure))
}

#[aoc_generator(day13, part2)]
fn read_schedule(input: &str) -> Result<Vec<(usize, usize)>, ParseError> {
    input
        .lines()
        .nth(1)
        .ok_or(ParseError::FormatError)?
        .split(',')
        .enumerate()
        .filter(|(_, item)| *item != "x")
        .map(|(offset, item)| Ok((offset, item.parse::<usize>()?)))
        .collect()
}

#[aoc(day13, part2)]
fn find_scheduled_departure(schedule: &[(usize, usize)]) -> usize {
    let mut timestamp = 0;
    let mut step = 1;
    for (offset, id) in schedule {
        while (timestamp + offset) % id != 0 {
            timestamp += step;
        }
        step *= id;
    }
    timestamp
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn parses_example() {
        assert_eq!(
            Notes::from_str(EXAMPLE),
            Ok(Notes {
                earliest_departure: 939,
                bus_ids: vec![7, 13, 59, 31, 19]
            })
        );
    }

    #[test]
    fn solve_example() {
        assert_eq!(
            Notes::from_str(EXAMPLE)
                .expect("failed to parse example")
                .find_first_departure(),
            Some((59, 944))
        );
        assert_eq!(
            multiply_departure_with_wait(
                &Notes::from_str(EXAMPLE).expect("failed to parse example")
            ),
            Some(295)
        );
    }

    const INPUT: &str = include_str!("../input/2020/day13.txt");

    #[test]
    fn solves_part1() {
        assert_eq!(
            multiply_departure_with_wait(&Notes::from_str(INPUT).expect("failed to parse input")),
            Some(2238)
        );
    }

    fn solve_part2(input: &str) -> Result<usize, ParseError> {
        read_schedule(input).map(|schedule| find_scheduled_departure(&schedule))
    }

    #[test]
    fn solve_part2_example() {
        assert_eq!(solve_part2("\n17,x,13,19"), Ok(3417));
        assert_eq!(solve_part2("\n67,7,59,61"), Ok(754018));
        assert_eq!(solve_part2("\n67,x,7,59,61"), Ok(779210));
        assert_eq!(solve_part2("\n67,7,x,59,61"), Ok(1261476));
        assert_eq!(solve_part2("\n1789,37,47,1889"), Ok(1202161486));
    }

    #[test]
    fn solve_part2() {
        assert_eq!(solve_part2(INPUT), Ok(560214575859998));
    }
}
