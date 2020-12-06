use anyhow::Result;
use std::io::{BufRead, BufReader, Read};

use itertools::__std_iter::FromIterator;
use itertools::{process_results, Itertools};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;

fn main() {
    let groups = read_groups(File::open("inputs/day06.txt").expect("Failed to open input"))
        .expect("Failed to read input");
    let any_yes_sum: usize = groups.iter().map(|group| count_any_yes(group.iter())).sum();
    println!("Got sum of any yes per group: {}", any_yes_sum);
    let agreements_sum: usize = groups
        .iter()
        .map(|group| count_agreements(group.iter()))
        .sum();
    println!("Got sum of agreements per group: {}", agreements_sum);
}

fn read_groups(input: impl Read) -> Result<Vec<Vec<String>>> {
    let input = BufReader::new(input).lines();
    process_results(input, |results| {
        results
            .peekable()
            .batching(|lines| match lines.peek() {
                Some(_) => Some(lines.take_while(|line| !line.is_empty()).collect_vec()),
                None => None,
            })
            .collect_vec()
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
        .unwrap_or_else(HashSet::new)
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
        let count: usize = read_groups(input.as_bytes())
            .unwrap()
            .iter()
            .map(|group| count_any_yes(group.iter()))
            .sum();
        assert_eq!(count, 11);
    }
}
