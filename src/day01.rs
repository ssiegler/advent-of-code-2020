use crate::read_lines;
use itertools::Itertools;
use std::num::ParseIntError;

const SUM: i32 = 2020;

#[aoc_generator(day1)]
fn read_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    read_lines(input)
}

#[aoc(day1, part1)]
fn part1(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .tuple_combinations()
        .find(|(a, b)| *a + *b == SUM)
        .map(|(a, b)| a * b)
}

#[aoc(day1, part2)]
fn part2(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .tuple_combinations()
        .find(|(a, b, c)| *a + *b + *c == SUM)
        .map(|(a, b, c)| a * b * c)
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
1721
979
366
299
675
1456";

    const INPUT: &str = include_str!("../input/2020/day1.txt");

    #[test]
    fn solve_example_part1() {
        assert_eq!(
            read_numbers(EXAMPLE_INPUT).map(|numbers| part1(&numbers)),
            Ok(Some(514579))
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(
            read_numbers(INPUT).map(|numbers| part1(&numbers)),
            Ok(Some(751776))
        );
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(
            read_numbers(EXAMPLE_INPUT).map(|numbers| part2(&numbers)),
            Ok(Some(241861950))
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            read_numbers(INPUT).map(|numbers| part2(&numbers)),
            Ok(Some(42275090))
        );
    }
}
