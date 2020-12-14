use std::num::ParseIntError;

use itertools::{Itertools, MinMaxResult};

use crate::read_lines;

#[aoc_generator(day9)]
fn read_numbers(input: &str) -> Result<Vec<u64>, ParseIntError> {
    read_lines(input)
}

#[aoc(day9, part1)]
fn find_first_xmas_mismatch(numbers: &[u64]) -> Option<u64> {
    find_first_mismatch(numbers, 25)
}

fn find_first_mismatch(numbers: &[u64], preamble_length: usize) -> Option<u64> {
    numbers
        .windows(preamble_length + 1)
        .find(|window| {
            window[0..preamble_length]
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != window[preamble_length])
        })
        .map(|window| window[preamble_length])
}

fn find_contiguous_sum(numbers: &[u64], target: u64) -> Option<&[u64]> {
    let mut start = 0;
    let mut sum = 0;
    for (end, number) in numbers.iter().enumerate() {
        sum += number;
        while sum > target {
            sum -= numbers[start];
            start += 1;
        }
        if sum == target {
            return Some(&numbers[start..=end]);
        }
    }
    None
}

#[aoc(day9, part2)]
fn part2(numbers: &[u64]) -> Option<u64> {
    find_contiguous_sum(numbers, 375054920).and_then(|slice| match slice.iter().minmax() {
        MinMaxResult::MinMax(min, max) => Some(min + max),
        _ => None,
    })
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    const INPUT: &str = include_str!("../input/2020/day9.txt");

    #[test]
    fn find_127_in_example() {
        let numbers: Vec<u64> = read_numbers(EXAMPLE).expect("Failed to read example");
        assert_eq!(find_first_mismatch(&numbers, 5), Some(127));
    }

    #[test]
    fn solve_part1() {
        assert_eq!(
            read_numbers(INPUT).map(|numbers| find_first_xmas_mismatch(&numbers)),
            Ok(Some(375054920))
        );
    }

    #[test]
    fn finds_sum_62_in_example() {
        assert_eq!(
            find_contiguous_sum(&read_numbers(EXAMPLE).expect("Failed to read example"), 127),
            Some(&[15, 25, 47, 40][..])
        );
    }

    #[test]
    fn solve_part2() {
        assert_eq!(
            read_numbers(INPUT).map(|numbers| part2(&numbers)),
            Ok(Some(54142584))
        );
    }
}
