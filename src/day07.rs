use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

use crate::{read_lines, ParseError};

type Bag = String;

struct Rule {
    container: Bag,
    contents: Vec<(usize, Bag)>,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (container, contents) = line
            .splitn(2, " bags contain ")
            .collect_tuple()
            .ok_or(ParseError::FormatError)?;
        let contents = read_counts(contents)?;
        Ok(Rule {
            container: container.to_string(),
            contents,
        })
    }
}

fn read_counts(input: &str) -> Result<Vec<(usize, String)>, ParseError> {
    Ok(if input == "no other bags." {
        vec![]
    } else {
        input
            .strip_suffix(".")
            .ok_or(ParseError::Missing("dot at end of rule"))?
            .split(", ")
            .map(read_count)
            .collect::<Result<_, _>>()?
    })
}

fn read_count(input: &str) -> Result<(usize, String), ParseError> {
    let parts = input.split_whitespace().collect_vec();
    if parts.len() != 4 || !matches!(parts[3], "bags" | "bag") {
        Err(ParseError::FormatError)
    } else {
        Ok((parts[0].parse()?, format!("{} {}", parts[1], parts[2])))
    }
}

#[aoc_generator(day7)]
fn read_rules(input: &str) -> Result<Vec<Rule>, ParseError> {
    read_lines(input)
}

#[aoc(day7, part1, Rules)]
fn part1_rules(rules: &[Rule]) -> usize {
    let mut bags: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("shiny gold".to_string());
    while let Some(bag) = queue.pop_front() {
        let containers = rules
            .iter()
            .filter(|rule| rule.contents.iter().any(|(_, content)| bag == *content))
            .map(|rule| rule.container.clone())
            .collect_vec();
        queue.extend(containers.iter().cloned());
        bags.extend(containers.iter().cloned());
    }
    bags.len()
}

#[aoc(day7, part1, Map)]
fn part1(rules: &[Rule]) -> usize {
    let contained_in: HashMap<Bag, Vec<Bag>> = rules
        .iter()
        .flat_map(|rule| {
            rule.contents
                .iter()
                .map(move |(_, bag)| (bag.clone(), rule.container.clone()))
        })
        .into_group_map();
    count_containment_options(&contained_in)
}

#[aoc_generator(day7, part1, direct)]
fn read_map(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .flat_map(|line| split_contained_pairs(line))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .into_group_map()
}

#[aoc(day7, part1, direct)]
fn count_containment_options(contained_in: &HashMap<String, Vec<String>>) -> usize {
    let mut bags: HashSet<Bag> = HashSet::new();
    let mut queue: VecDeque<Bag> = VecDeque::new();
    queue.push_back("shiny gold".to_string());
    while let Some(bag) = queue.pop_front() {
        if let Some(containers) = contained_in.get(&bag) {
            queue.extend(
                containers
                    .iter()
                    .filter(|container| !bags.contains(*container))
                    .cloned(),
            );
            bags.extend(containers.iter().cloned());
        }
    }
    bags.len()
}

fn split_contained_pairs(rule: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    let mut parts = rule.splitn(2, " bags contain ");
    let container = parts.next().unwrap();
    let parts = parts.next().unwrap_or("").trim_end_matches('.');
    parts
        .split(", ")
        .map(move |part| (normalize(part), container))
}

fn normalize(part: &str) -> &str {
    part.trim_end_matches("bag")
        .trim_end_matches("bags")
        .trim_start_matches(|ch: char| ch.is_digit(10))
        .trim()
}

fn count_bags(rules: &[Rule], bag: &str) -> usize {
    rules
        .iter()
        .find(|rule| rule.container == *bag)
        .map(|rule| {
            rule.contents
                .iter()
                .map(|(count, bag)| count + (count_bags(rules, bag) * count))
                .sum::<usize>()
        })
        .unwrap_or(0)
}

#[aoc(day7, part2)]
fn part2(rules: &[Rule]) -> usize {
    count_bags(&rules, "shiny gold")
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    const INPUT: &str = include_str!("../input/2020/day7.txt");

    fn solve(solver: fn(&[Rule]) -> usize, input: &str) -> Result<usize, ParseError> {
        read_rules(input).map(|rules| solver(&rules))
    }

    #[test]
    fn solve_example() {
        assert_eq!(solve(part1, EXAMPLE), Ok(4));
    }

    #[test]
    fn solve_example_without_map() {
        assert_eq!(solve(part1_rules, EXAMPLE), Ok(4));
    }
    #[test]
    fn solve_example_without_parsing() {
        assert_eq!(count_containment_options(&read_map(EXAMPLE)), 4);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(solve(part1, INPUT), Ok(155));
    }

    #[test]
    fn solve_part1_without_map() {
        assert_eq!(solve(part1_rules, INPUT), Ok(155));
    }

    #[test]
    fn solve_part1_without_parsing() {
        assert_eq!(count_containment_options(&read_map(INPUT)), 155);
    }

    #[test]
    fn counts_bags_in_example() {
        let rules = read_rules(EXAMPLE).expect("Failed to read example rules");
        assert_eq!(count_bags(&rules, "faded blue"), 0);
        assert_eq!(count_bags(&rules, "dotted black"), 0);
        assert_eq!(count_bags(&rules, "vibrant plum"), 11);
        assert_eq!(count_bags(&rules, "dark olive"), 7);
        assert_eq!(count_bags(&rules, "shiny gold"), 32);
    }

    #[test]
    fn solve_example_part2() {
        assert_eq!(solve(part2, EXAMPLE), Ok(32));
    }

    #[test]
    fn solve_second_example() {
        assert_eq!(solve(part2, EXAMPLE2), Ok(126));
    }

    #[test]
    fn solve_part2() {
        assert_eq!(solve(part2, INPUT), Ok(54803));
    }
}
