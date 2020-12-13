use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

fn count_questions_with_any_yes(group: &str) -> usize {
    group
        .lines()
        .flat_map(|answers| answers.chars())
        .unique()
        .count()
}

fn count_agreements(group: &str) -> usize {
    group
        .lines()
        .map(|answers| HashSet::from_iter(answers.chars()))
        .fold1(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or_else(HashSet::new)
        .len()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input.split("\n\n").map(count_questions_with_any_yes).sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    input.split("\n\n").map(count_agreements).sum()
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn counts_any_yes() {
        assert_eq!(
            count_questions_with_any_yes(
                "\
abcx
abcy
abcz"
            ),
            6
        );
    }

    #[test]
    fn counts_agreements() {
        assert_eq!(
            ["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]
                .iter()
                .map(|group| count_agreements(group))
                .collect_vec(),
            vec![3, 0, 1, 1, 1]
        );
    }

    const INPUT: &str = include_str!("../input/2020/day6.txt");

    #[test]
    fn solve_example_part1() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(INPUT), 6590);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(INPUT), 3288);
    }
}
