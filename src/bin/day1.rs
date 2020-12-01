use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const TARGET_SUM: i32 = 2020;

fn main() {
    let input = File::open("inputs/day1.txt").expect("Failed to open input file");
    let input : Vec<i32> = BufReader::new(input)
        .lines().map(|line| line.expect("Failed to read line").parse::<i32>().expect("failed to parse line as number")).collect();
    let (a, b) = find_first_pair_with_sum(input.as_slice(), TARGET_SUM).expect("No pair found");
    println!("{} * {} = {}", a, b, a * b);
}

fn find_first_pair_with_sum(numbers: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut seen = HashSet::new();
    for &number in numbers {
        let partner = sum - number;
        if seen.contains(&partner) {
            return Some((partner, number));
        }
        seen.insert(number);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[i32] = &[
        1721,
        979,
        366,
        299,
        675,
        1456];

    #[test]
    fn finds_first_pair_for_sum() {
        assert_eq!(find_first_pair_with_sum(EXAMPLE_INPUT, TARGET_SUM), Some((1721, 299)));
    }

}

