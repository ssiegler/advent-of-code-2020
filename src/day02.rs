use std::str::FromStr;

use crate::{read_lines, ParseError};

struct Password {
    low: usize,
    high: usize,
    letter: char,
    word: String,
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.splitn(2, ": ");
        let (low, high, letter) = split_policy(parts.next().unwrap())?;
        let word = parts.next().ok_or(ParseError::Missing("word"))?.to_string();
        Ok(Password {
            low,
            high,
            letter,
            word,
        })
    }
}

fn split_policy(input: &str) -> Result<(usize, usize, char), ParseError> {
    let mut policy = input.splitn(3, &[' ', '-'][..]);
    Ok((
        policy
            .next()
            .ok_or(ParseError::Missing("low"))?
            .parse::<usize>()?,
        policy
            .next()
            .ok_or(ParseError::Missing("low"))?
            .parse::<usize>()?,
        policy
            .next()
            .ok_or(ParseError::Missing("low"))?
            .parse::<char>()?,
    ))
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.word.chars().filter(|ch| *ch == self.letter).count();
        count >= self.low && count <= self.high
    }

    fn is_officially_valid(&self) -> bool {
        self.word
            .get(self.low - 1..self.high)
            .map(|substring| substring.starts_with(self.letter) != substring.ends_with(self.letter))
            .unwrap_or(false)
    }
}

#[aoc_generator(day2)]
fn read_passwords(input: &str) -> Result<Vec<Password>, ParseError> {
    read_lines(input)
}

#[aoc(day2, part1)]
fn part1(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|password| password.is_valid())
        .count()
}

#[aoc(day2, part2)]
fn part2(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|password| password.is_officially_valid())
        .count()
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    const INPUT: &str = include_str!("../input/2020/day2.txt");

    #[test]
    fn solves_example_part1() {
        assert_eq!(
            read_passwords(EXAMPLE).map(|passwords| part1(&passwords)),
            Ok(2)
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            read_passwords(INPUT).map(|passwords| part1(&passwords)),
            Ok(603)
        );
    }

    #[test]
    fn solves_example_part2() {
        assert_eq!(
            read_passwords(EXAMPLE).map(|passwords| part2(&passwords)),
            Ok(1)
        );
    }
}
