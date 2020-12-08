use crate::puzzle::{Input, Puzzle};
use itertools::Itertools;

type Day01 = Input<Vec<i32>>;

impl Day01 {
    const SUM: i32 = 2020;

    fn find_product_of_sum(&self, length: usize) -> Option<i32> {
        self.iter()
            .cloned()
            .combinations(length)
            .find(|combination| combination.iter().sum::<i32>() == Self::SUM)
            .map(|combination| combination.iter().product())
    }
}

impl Puzzle for Day01 {
    fn solve_part1(&self) -> String {
        self.find_product_of_sum(2).map_or_else(
            || "No pair found".to_string(),
            |product| product.to_string(),
        )
    }

    fn solve_part2(&self) -> String {
        self.find_product_of_sum(3).map_or_else(
            || "No triple found".to_string(),
            |product| product.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EXAMPLE: Day01 = "\
1721
979
366
299
675
1456"
            .parse()
            .expect("Failed to parse example");
        static ref PUZZLE: Day01 =
            Day01::load("inputs/day01.txt").expect("Error loading puzzle input");
    }
    #[test]
    fn solves_example_part1() {
        assert_eq!(EXAMPLE.solve_part1(), "514579")
    }

    #[test]
    fn solves_part1() {
        assert_eq!(PUZZLE.solve_part1(), "751776");
    }

    #[test]
    fn solves_example_part2() {
        assert_eq!(EXAMPLE.solve_part2(), "241861950");
    }

    #[test]
    fn solves_part2() {
        assert_eq!(PUZZLE.solve_part2(), "42275090");
    }
}
