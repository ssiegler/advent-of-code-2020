struct Slope {
    down: usize,
    right: usize,
}

const SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

fn count_trees(input: &str, slope: &Slope) -> usize {
    input
        .lines()
        .step_by(slope.down)
        .enumerate()
        .filter(|(step, row)| row.chars().cycle().nth(step * slope.right).unwrap() == '#')
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    count_trees(input, &Slope { right: 3, down: 1 })
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    SLOPES
        .iter()
        .map(|slope| count_trees(input, slope))
        .product()
}

#[cfg(test)]
mod should {
    use super::*;
    use itertools::Itertools;

    const EXAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    const INPUT: &str = include_str!("../input/2020/day3.txt");

    #[test]
    fn count_7_trees_in_example_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(INPUT), 234);
    }

    #[test]
    fn count_trees_on_slopes() {
        assert_eq!(
            SLOPES
                .iter()
                .map(|slope| count_trees(EXAMPLE, slope))
                .collect_vec(),
            &[2, 7, 3, 4, 2]
        );
    }
    #[test]
    fn solves_part2() {
        assert_eq!(part2(INPUT), 5813773056);
    }
}
