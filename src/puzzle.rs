use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub trait Puzzle: FromStr {
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;

    fn load(path: impl AsRef<Path>) -> Result<Self, PuzzleError>
    where
        Self: Sized,
    {
        let input = std::fs::read_to_string(path)?;
        input.parse().map_err(|_| PuzzleError::InvalidInput)
    }
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
pub enum PuzzleError {
    #[error("Error reading from input file")]
    ReadFailed(#[from] std::io::Error),
    #[error("Error parsing puzzle input")]
    InvalidInput,
}
