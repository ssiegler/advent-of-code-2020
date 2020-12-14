use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[aoc_generator(day7)]
fn read_containers(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .flat_map(|line| split_contained_pairs(line))
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .into_group_map()
}

#[aoc(day7, part1)]
fn part1(contained_in: &HashMap<String, Vec<String>>) -> usize {
    let mut bags: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
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

    #[test]
    fn normalizes_bag_specs() {
        assert_eq!(normalize("1 bright white bag"), "bright white");
        assert_eq!(normalize("6 dotted black bags"), "dotted black");
    }

    #[test]
    fn solve_example() {
        assert_eq!(part1(&read_containers(EXAMPLE)), 4);
    }

    const INPUT: &str = include_str!("../input/2020/day7.txt");

    #[test]
    fn solve_part1() {
        assert_eq!(part1(&read_containers(INPUT)), 155);
    }
}
