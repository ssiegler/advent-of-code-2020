use crate::{read_lines, ParseError};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
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

#[derive(Debug, PartialEq)]
enum ExecutionResult {
    Terminated(isize),
    Stopped(isize),
}

#[cfg(test)]
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

#[cfg(test)]
const INPUT: &str = include_str!("../input/2020/day8.txt");

mod trivial {
    use super::*;

    #[aoc(day8, part1, trivial)]
    fn execute_until_loop(input: &str) -> Result<isize, ProgramError> {
        match run_first_iteration(input)? {
            ExecutionResult::Terminated(_) => Err(ProgramError::UnexpectedTermination),
            ExecutionResult::Stopped(value) => Ok(value),
        }
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
            let argument = argument.parse::<isize>().map_err(ParseError::from)?;
            match operation {
                "acc" => {
                    accumulator += argument;
                    position += 1;
                }
                "jmp" => position = (position as isize + argument) as usize,
                "nop" => position += 1,
                _ => return Err(ProgramError::UnknownOperation(operation.to_string())),
            }
            if !executed_positions.insert(position) {
                return Ok(ExecutionResult::Stopped(accumulator));
            }
        }
        Ok(ExecutionResult::Terminated(accumulator))
    }

    fn fix_program(input: &str) -> impl Iterator<Item = String> + '_ {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"jmp|nop").unwrap();
        }
        RE.find_iter(input).map(move |m| {
            let mut fixed_program = input.to_string();
            fixed_program.replace_range(m.range(), if m.as_str() == "jmp" { "nop" } else { "jmp" });
            fixed_program
        })
    }

    #[aoc(day8, part2, trivial)]
    fn find_termination_fix(input: &str) -> Option<isize> {
        fix_program(input)
            .map(|fix| run_first_iteration(&fix))
            .find_map(|result| {
                result.ok().and_then(|result| match result {
                    ExecutionResult::Terminated(result) => Some(result),
                    ExecutionResult::Stopped(_) => None,
                })
            })
    }

    #[cfg(test)]
    mod should {
        use super::*;

        #[test]
        fn have_accumulator_of_5_after_first_iteration() {
            assert_eq!(execute_until_loop(EXAMPLE), Ok(5));
        }

        #[test]
        fn solve_part1() {
            assert_eq!(execute_until_loop(INPUT), Ok(1801));
        }

        #[test]
        fn finds_fix_for_example() {
            assert_eq!(find_termination_fix(EXAMPLE), Some(8));
        }
        #[test]
        fn solve_part2() {
            assert_eq!(find_termination_fix(INPUT), Some(2060));
        }
    }
}

mod parsed {
    use super::*;

    #[aoc_generator(day8)]
    fn read_program(input: &str) -> Result<Vec<Instruction>, ProgramError> {
        read_lines(input)
    }

    fn run_first_iteration(program: &[Instruction]) -> ExecutionResult {
        let mut executed_positions = HashSet::new();
        let mut accumulator = 0;
        let mut position = 0;
        while position < program.len() {
            let instruction = &program[position];
            match instruction.operation {
                Operation::Acc => {
                    accumulator += instruction.argument;
                    position += 1;
                }
                Operation::Jmp => position = (position as isize + instruction.argument) as usize,
                Operation::Nop => position += 1,
            }
            if !executed_positions.insert(position) {
                return ExecutionResult::Stopped(accumulator);
            }
        }
        ExecutionResult::Terminated(accumulator)
    }

    #[aoc(day8, part1)]
    fn execute_until_loop(program: &[Instruction]) -> Option<isize> {
        match run_first_iteration(program) {
            ExecutionResult::Terminated(_) => None,
            ExecutionResult::Stopped(value) => Some(value),
        }
    }

    fn fix_program(program: &[Instruction]) -> impl Iterator<Item = Vec<Instruction>> + '_ {
        program
            .iter()
            .enumerate()
            .filter_map(move |(position, instruction)| match instruction.operation {
                // Converting a nop to a jmp 1 changes nothing. Converting a nop to a jmp 0 creates an endless loop.
                Operation::Nop if instruction.argument != 1 && instruction.argument != 0 => {
                    Some(replace_operation(program, position, Operation::Jmp))
                }
                // Converting a jmp 1 to a nop changes nothing
                Operation::Jmp if instruction.argument != 1 => {
                    Some(replace_operation(program, position, Operation::Nop))
                }
                _ => None,
            })
    }

    fn replace_operation(
        program: &[Instruction],
        position: usize,
        operation: Operation,
    ) -> Vec<Instruction> {
        let mut program = program.to_vec();
        program[position] = Instruction {
            operation,
            argument: program[position].argument,
        };
        program
    }

    #[aoc(day8, part2)]
    fn find_termination_fix(program: &[Instruction]) -> Option<isize> {
        fix_program(program)
            .map(|fix| run_first_iteration(&fix))
            .find_map(|result| match result {
                ExecutionResult::Terminated(result) => Some(result),
                ExecutionResult::Stopped(_) => None,
            })
    }

    #[derive(Copy, Clone, Debug)]
    enum Operation {
        Nop,
        Acc,
        Jmp,
    }

    #[derive(Copy, Clone, Debug)]
    struct Instruction {
        operation: Operation,
        argument: isize,
    }

    impl FromStr for Instruction {
        type Err = ProgramError;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let (operation, argument) = input
                .split_whitespace()
                .collect_tuple()
                .ok_or(ParseError::FormatError)?;
            let argument = argument.parse::<isize>().map_err(ParseError::from)?;
            let operation = match operation {
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                "nop" => Operation::Nop,
                _ => return Err(ProgramError::UnknownOperation(operation.to_string())),
            };
            Ok(Instruction {
                operation,
                argument,
            })
        }
    }

    #[cfg(test)]
    mod should {
        use super::*;

        #[test]
        fn have_accumulator_of_5_after_first_iteration() {
            assert_eq!(
                read_program(EXAMPLE).map(|program| execute_until_loop(&program)),
                Ok(Some(5))
            );
        }

        #[test]
        fn solve_part1() {
            assert_eq!(
                read_program(INPUT).map(|program| execute_until_loop(&program)),
                Ok(Some(1801))
            );
        }

        #[test]
        fn finds_fix_for_example() {
            assert_eq!(
                read_program(EXAMPLE).map(|program| find_termination_fix(&program)),
                Ok(Some(8))
            );
        }
        #[test]
        fn solve_part2() {
            assert_eq!(
                read_program(INPUT).map(|program| find_termination_fix(&program)),
                Ok(Some(2060))
            );
        }
    }
}
