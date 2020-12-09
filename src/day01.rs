use crate::puzzle::{Lines, Puzzle};
use itertools::Itertools;

type Day01 = Lines<i32>;

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

test_puzzle! {
    Day01;
    Example("\
1721
979
366
299
675
1456", 514579, 241861950),
    File("inputs/day01.txt", 751776, 42275090)
}
