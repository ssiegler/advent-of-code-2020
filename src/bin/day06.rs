use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

use itertools::__std_iter::FromIterator;
use itertools::{process_results, Itertools};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;

fn main() {
    let answer_sum =
        summed_group_answers(File::open("inputs/day06.txt").expect("Failed to open input"))
            .expect("Failed to read input");
    println!("Got sum of group counts: {}", answer_sum);
}

fn summed_group_answers(input: impl Read) -> Result<usize> {
    let input = BufReader::new(input).lines();
    process_results(input, |results| {
        results
            .peekable()
            .batching(|lines| match lines.peek() {
                Some(_) => Some(count_any_yes(lines.take_while(|line| !line.is_empty()))),
                None => None,
            })
            .sum()
    })
    .map_err(anyhow::Error::from)
}

fn count_any_yes<I>(mut group: I) -> usize
where
    I: Iterator,
    I::Item: Display,
{
    group.join("").chars().unique().count()
}

fn count_agreements<I>(group: I) -> usize
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    group
        .map(|answers| HashSet::from_iter(answers.as_ref().chars()))
        .fold1(|a, b| a.intersection(&b).cloned().collect())
        .unwrap_or(HashSet::new())
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_agreements_in_example() {
        let example = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];
        itertools::assert_equal(
            example.iter().map(|group| count_agreements(group.iter())),
            vec![3, 0, 1, 1, 1],
        );
    }

    #[test]
    fn counts_any_yes_in_example() {
        let example = ["abcx", "abcy", "abcz"];
        let count = count_any_yes(example.iter());
        assert_eq!(count, 6);
    }

    #[test]
    fn counts_yes_answers_for_groups() {
        let input = "\
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
        assert_eq!(summed_group_answers(input.as_bytes()).unwrap(), 11);
    }
}
