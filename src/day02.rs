use crate::day02::PasswordError::{InvalidField, MissingField, RecordMismatch};
use crate::puzzle::{Lines, Puzzle};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::str::FromStr;
use thiserror::Error;

type Day02 = Lines<Password>;

impl Puzzle for Day02 {
    fn solve_part1(&self) -> String {
        self.iter()
            .filter(|password| password.is_valid())
            .count()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.iter()
            .filter(|password| password.is_officially_valid())
            .count()
            .to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|ch| *ch == self.letter)
            .count();
        count >= self.min && count <= self.max
    }

    fn is_officially_valid(&self) -> bool {
        match (
            self.password
                .chars()
                .nth(self.min - 1)
                .map(|ch| ch == self.letter),
            self.password
                .chars()
                .nth(self.max - 1)
                .map(|ch| ch == self.letter),
        ) {
            (Some(true), Some(false)) => true,
            (Some(false), Some(true)) => true,
            _ => false,
        }
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            ^(?P<min>[\d]+)
            -
            (?P<max>[\d]+)
            \s
            (?P<letter>[\w])
            :\s
            (?P<password>[\w]+)$
            "
            )
            .unwrap();
        }
        let cap = RE
            .captures(input)
            .ok_or_else(|| RecordMismatch(input.to_string()))?;
        let (min, max, letter, password) = (
            from_named(&cap, "min")?,
            from_named(&cap, "max")?,
            from_named(&cap, "letter")?,
            from_named(&cap, "password")?,
        );
        Ok(Password {
            min,
            max,
            letter,
            password,
        })
    }
}

fn from_named<T: FromStr>(cap: &Captures, name: &str) -> Result<T, PasswordError> {
    let value = cap
        .name(name)
        .ok_or_else(|| MissingField(name.to_string()))?
        .as_str();
    value.parse().map_err(|_| InvalidField {
        name: name.to_string(),
        value: value.to_string(),
    })
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("The field `{0}` is missing")]
    MissingField(String),
    #[error("Value {name:?} invalid for field `{name}`")]
    InvalidField { name: String, value: String },
    #[error("Record does not fit password fields: {0:?}")]
    RecordMismatch(String),
}

test_puzzle!(
    Day02;
    Example("\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc", 2, 1),
    File("inputs/day02.txt", 603, 404)
);
