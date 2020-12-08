use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub trait Puzzle: FromStr {
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub fn load<T: FromStr>(path: impl AsRef<Path>) -> Result<T, LoadError> {
    let input = std::fs::read_to_string(path)?;
    input.parse().map_err(|_| LoadError::InvalidInput)
}

pub struct Input<T>(T);

impl<T> Deref for Input<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FromStr> FromStr for Input<Vec<T>> {
    type Err = T::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Input(
            input
                .lines()
                .map(|line| line.parse())
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Error reading from input file")]
    ReadFailed(#[from] std::io::Error),
    #[error("Error parsing puzzle input")]
    InvalidInput,
}
