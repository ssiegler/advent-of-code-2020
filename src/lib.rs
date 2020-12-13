#[macro_use]
extern crate aoc_runner_derive;
use std::str::FromStr;

pub fn read_lines<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    input.lines().map(|line| line.parse()).collect()
}

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

aoc_lib! { year = 2020 }
