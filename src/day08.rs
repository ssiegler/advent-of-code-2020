use crate::ParseError;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
enum ProgramError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error("Programm terminated unexpectedly")]
    UnexpectedTermination,
    #[error("Unknown operation: {0}")]
    UnknownOperation(String),
}

#[aoc(day8, part1, trivial)]
fn part1(input: &str) -> Result<i32, ProgramError> {
    match run_first_iteration(input)? {
        ExecutionResult::Terminated(_) => Err(ProgramError::UnexpectedTermination),
        ExecutionResult::Stopped(value) => Ok(value),
    }
}

enum ExecutionResult {
    Terminated(i32),
    Stopped(i32),
}

fn run_first_iteration(input: &str) -> Result<ExecutionResult, ProgramError> {
    let program = input.lines().collect_vec();
    let mut executed_positions = HashSet::new();
    let mut accumulator = 0;
    let mut position = 0;
    while position < program.len() {
        let (operation, argument) = program[position]
            .split_whitespace()
            .collect_tuple()
            .ok_or(ParseError::FormatError)?;
        let argument = argument.parse::<i32>().map_err(ParseError::from)?;
        match operation {
            "acc" => {
                accumulator += argument;
                position += 1;
            }
            "jmp" => position = (position as i32 + argument) as usize,
            "nop" => position += 1,
            _ => return Err(ProgramError::UnknownOperation(operation.to_string())),
        }
        if !executed_positions.insert(position) {
            return Ok(ExecutionResult::Stopped(accumulator));
        }
    }
    Ok(ExecutionResult::Terminated(accumulator))
}

fn fix_programm(input: &str) -> impl Iterator<Item = String> + '_ {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"jmp|nop").unwrap();
    }
    RE.find_iter(input).map(move |m| {
        let mut fixed_program = input.to_string();
        fixed_program.replace_range(m.range(), if m.as_str() == "jmp" { "nop" } else { "jmp" });
        fixed_program
    })
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const INPUT: &str = include_str!("../input/2020/day8.txt");

    #[test]
    fn have_accumulator_of_5_after_first_iteration() {
        assert_eq!(part1(EXAMPLE), Ok(5));
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(INPUT), Ok(1801));
    }
}
