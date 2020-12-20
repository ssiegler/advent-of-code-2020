#[macro_use]
extern crate aoc_runner_derive;
use std::char::ParseCharError;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub fn read_lines<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Format mismatch")]
    FormatError,
    #[error("Missing: {0}")]
    Missing(&'static str),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
    #[error(transparent)]
    InvalidLetter(#[from] ParseCharError),
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

aoc_lib! { year = 2020 }
