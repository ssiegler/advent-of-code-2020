use crate::read_lines;
use itertools::Itertools;
use std::iter::once;
use std::num::ParseIntError;

fn find_jolt_differences(joltages: &[usize]) -> (usize, usize, usize) {
    once(&0)
        .chain(joltages.iter().sorted())
        .tuple_windows()
        .map(|(a, b)| (b - a) as usize)
        .chain(once(3))
        .fold((0, 0, 0), |(a, b, c), diff| match diff {
            1 => (a + 1, b, c),
            2 => (a, b + 1, c),
            3 => (a, b, c + 1),
            _ => (a, b, c),
        })
}

#[aoc_generator(day10)]
fn read_numbers(input: &str) -> Result<Vec<usize>, ParseIntError> {
    read_lines(input)
}

#[aoc(day10, part1)]
fn part1(numbers: &[usize]) -> usize {
    let (a, _, b) = find_jolt_differences(numbers);
    a * b
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &[usize] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    #[test]
    fn finds_jolt_differences_in_example() {
        assert_eq!(find_jolt_differences(EXAMPLE), (7, 0, 5));
    }

    const LARGER_EXAMPLE: &[usize] = &[
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn finds_jolt_differences_in_larger_example() {
        assert_eq!(find_jolt_differences(LARGER_EXAMPLE), (22, 0, 10));
    }

    const INPUT: &str = include_str!("../input/2020/day10.txt");

    #[test]
    fn solve_part1() {
        assert_eq!(read_numbers(INPUT).map(|numbers| part1(&numbers)), Ok(1980));
    }
}
