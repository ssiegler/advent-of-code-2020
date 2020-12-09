use crate::puzzle::{Blocks, Puzzle};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

type Day06 = Blocks<GroupAnswers>;

struct GroupAnswers(Vec<String>);

impl GroupAnswers {
    fn count_questions_with_any_yes(&self) -> usize {
        self.0
            .iter()
            .flat_map(|answers| answers.chars())
            .unique()
            .count()
    }

    fn count_agreements(&self) -> usize {
        self.0
            .iter()
            .map(|answers| HashSet::from_iter(answers.chars()))
            .fold1(|a, b| a.intersection(&b).cloned().collect())
            .unwrap_or_else(HashSet::new)
            .len()
    }
}

impl FromStr for GroupAnswers {
    type Err = ();

    fn from_str(group_answers: &str) -> Result<Self, Self::Err> {
        Ok(GroupAnswers(
            group_answers.lines().map(String::from).collect_vec(),
        ))
    }
}

impl Puzzle for Day06 {
    fn solve_part1(&self) -> String {
        self.iter()
            .map(|group_answers| group_answers.count_questions_with_any_yes())
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.iter()
            .map(|group_answers| group_answers.count_agreements())
            .sum::<usize>()
            .to_string()
    }
}

test_puzzle!(Day06;
Example("\
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

b", 11, 6),
File("inputs/day06.txt", 6590, 3288)
);

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn counts_any_yes() {
        let example = GroupAnswers(vec![
            "abcx".to_string(),
            "abcy".to_string(),
            "abcz".to_string(),
        ]);
        let count = example.count_questions_with_any_yes();
        assert_eq!(count, 6);
    }

    #[test]
    fn counts_agreements() {
        let example = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ]
        .iter()
        .map(|group| {
            GroupAnswers(
                group
                    .iter()
                    .map(|answers| answers.to_string())
                    .collect_vec(),
            )
        })
        .collect_vec();
        itertools::assert_equal(
            example.iter().map(|group| group.count_agreements()),
            vec![3, 0, 1, 1, 1],
        );
    }
}
