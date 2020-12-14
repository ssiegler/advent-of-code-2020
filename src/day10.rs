use std::iter::once;
use std::num::ParseIntError;

use itertools::process_results;
use itertools::Itertools;

fn find_jolt_differences(joltages: &[u32]) -> (u32, u32, u32) {
    joltages
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b - a) as u32)
        .fold((0, 0, 0), |(a, b, c), diff| match diff {
            1 => (a + 1, b, c),
            2 => (a, b + 1, c),
            3 => (a, b, c + 1),
            _ => (a, b, c),
        })
}

#[aoc_generator(day10)]
fn sorted_joltages(input: &str) -> Result<Vec<u32>, ParseIntError> {
    process_results(input.lines().map(|line| line.parse::<u32>()), |numbers| {
        let mut joltages = once(0).chain(numbers).sorted().collect_vec();
        joltages.push(joltages.last().unwrap() + 3);
        joltages
    })
}

#[aoc(day10, part1)]
fn part1(numbers: &[u32]) -> u32 {
    let (a, _, b) = find_jolt_differences(numbers);
    a * b
}

#[aoc(day10, part2)]
fn count_combinations(joltages: &[u32]) -> usize {
    let mut counts = vec![0; joltages.len()];
    counts[0] = 1;
    for (offset, current) in joltages.iter().enumerate() {
        for (index, _) in (1..).zip(
            joltages[offset..]
                .iter()
                .skip(1)
                .take_while(|next| **next - *current <= 3),
        ) {
            counts[index + offset] += counts[offset];
        }
    }
    counts.last().cloned().unwrap_or(0)
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &[u32] = &[0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];

    #[test]
    fn finds_jolt_differences_in_example() {
        assert_eq!(find_jolt_differences(EXAMPLE), (7, 0, 5));
    }

    const LARGER_EXAMPLE: &[u32] = &[
        0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49, 52,
    ];

    #[test]
    fn finds_jolt_differences_in_larger_example() {
        assert_eq!(find_jolt_differences(LARGER_EXAMPLE), (22, 0, 10));
    }

    const INPUT: &str = include_str!("../input/2020/day10.txt");

    #[test]
    fn solve_part1() {
        assert_eq!(
            sorted_joltages(INPUT).map(|numbers| part1(&numbers)),
            Ok(1980)
        );
    }

    #[test]
    fn counts_8_combinations_in_example() {
        assert_eq!(count_combinations(EXAMPLE), 8);
    }

    #[test]
    fn counts_19208_combinations_in_larger_example() {
        assert_eq!(count_combinations(LARGER_EXAMPLE), 19208);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(
            sorted_joltages(INPUT).map(|numbers| count_combinations(&numbers)),
            Ok(4628074479616)
        );
    }
}
