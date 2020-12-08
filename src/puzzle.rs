use std::ops::Deref;
use std::str::FromStr;

pub trait Puzzle: FromStr {
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
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

macro_rules! test_puzzle {
    ($type:ty;
        Example($example_input:literal, $ex_part1:literal, $ex_part2:literal),
        File($path:literal, $part1:literal, $part2:literal)
    ) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use lazy_static::lazy_static;

            lazy_static! {
                static ref EXAMPLE: $type = $example_input
                    .parse::<$type>()
                    .expect("Failed to parse example");
                static ref PUZZLE: $type = std::fs::read_to_string($path)
                    .expect("Failed to read input")
                    .parse::<$type>()
                    .expect("Failed to parse input");
            }

            #[test]
            fn solves_example_part1() {
                assert_eq!(EXAMPLE.solve_part1(), $ex_part1.to_string())
            }
            #[test]
            fn solves_example_part2() {
                assert_eq!(EXAMPLE.solve_part2(), $ex_part2.to_string())
            }
            #[test]
            fn solves_part1() {
                assert_eq!(PUZZLE.solve_part1(), $part1.to_string())
            }
            #[test]
            fn solves_part2() {
                assert_eq!(PUZZLE.solve_part2(), $part2.to_string())
            }
        }
    };
}
