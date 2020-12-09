use crate::puzzle::{Lines, Puzzle};

type Day03 = Lines<String>;

struct Slope {
    down: usize,
    right: usize,
}

impl Day03 {
    fn count_trees(&self, slope: &Slope) -> usize {
        self.iter()
            .step_by(slope.down)
            .enumerate()
            .filter(|(step, row)| row.chars().cycle().nth(step * slope.right).unwrap() == '#')
            .count()
    }
}

impl Puzzle for Day03 {
    fn solve_part1(&self) -> String {
        self.count_trees(&Slope { down: 1, right: 3 }).to_string()
    }

    fn solve_part2(&self) -> String {
        [
            Slope { right: 1, down: 1 },
            Slope { right: 3, down: 1 },
            Slope { right: 5, down: 1 },
            Slope { right: 7, down: 1 },
            Slope { right: 1, down: 2 },
        ]
        .iter()
        .map(|slope| self.count_trees(&slope))
        .product::<usize>()
        .to_string()
    }
}

test_puzzle!(Day03;
    Example("\
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
.#..#...#.#", 7, 336),
    File("inputs/day03.txt", 234, 5813773056usize)
);
