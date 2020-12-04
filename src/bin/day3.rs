use advent_of_code::read_lines;

const SLOPES: &[Slope] = &[
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
];

fn main() {
    let tree_count = count_trees(read_lines("day03/input.txt"), &SLOPES[1]);
    println!("Encountered {} trees", tree_count);
    let answer: usize = SLOPES
        .iter()
        .map(|slope| count_trees(read_lines("day03/input.txt"), slope))
        .product();
    println!("Product of encountered trees: {}", answer);
}

fn count_trees(tree_rows: impl Iterator<Item = impl AsRef<str>>, slope: &Slope) -> usize {
    tree_rows
        .step_by(slope.down)
        .enumerate()
        .filter(|(step, row)| {
            row.as_ref()
                .chars()
                .cycle()
                .nth(step * slope.right)
                .unwrap()
                == '#'
        })
        .count()
}

struct Slope {
    right: usize,
    down: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
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

    #[test]
    fn encountered_7_trees_in_example_with_slope_3_right_1_down() {
        let tree_count = count_trees(EXAMPLE_INPUT.lines(), &Slope { right: 3, down: 1 });
        assert_eq!(tree_count, 7);
    }

    #[test]
    fn encountered_trees_with_scopes_in_example() {
        let tree_counts: Vec<usize> = SLOPES
            .iter()
            .map(|slope| count_trees(EXAMPLE_INPUT.lines(), slope))
            .collect();
        assert_eq!(tree_counts, &[2, 7, 3, 4, 2]);
    }
}
