use advent_of_code::read_numbers_from_file;

const TARGET_SUM: i32 = 2020;

fn main() {
    let input = read_numbers_from_file("inputs/day1.txt");
    let (a, b) = find_first_pair_with_sum(&input, TARGET_SUM).expect("No pair found");
    println!("{} * {} = {}", a, b, a * b);
    let (a, b, c) = find_first_triplet_with_sum(&input, TARGET_SUM).expect("No triplet found");
    println!("{} * {} * {} = {}", a, b, c, a * b * c);
}

fn find(numbers: &[i32], needle: i32) -> Option<i32> {
    numbers.iter().find(|number| **number == needle).cloned()
}

fn find_first_pair_with_sum(numbers: &[i32], sum: i32) -> Option<(i32, i32)> {
    numbers.iter().enumerate().find_map(|(index, &number)| {
        find(&numbers[index..], sum - number).map(|candidate| (number, candidate))
    })
}

fn find_first_triplet_with_sum(numbers: &[i32], sum: i32) -> Option<(i32, i32, i32)> {
    numbers.iter().enumerate().find_map(|(index, &number)| {
        find_first_pair_with_sum(&numbers[index..], sum - number).map(|(a, b)| (number, a, b))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[i32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn finds_first_pair_for_sum() {
        assert_eq!(
            find_first_pair_with_sum(EXAMPLE_INPUT, TARGET_SUM),
            Some((1721, 299))
        );
    }

    #[test]
    fn find_first_triplet_for_sum() {
        assert_eq!(
            find_first_triplet_with_sum(EXAMPLE_INPUT, TARGET_SUM),
            Some((979, 366, 675))
        )
    }
}
