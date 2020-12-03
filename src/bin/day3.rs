use advent_of_code::read_lines;

fn main() {
    let tree_count: usize = count_trees(
        read_lines("inputs/day3.txt".as_ref()),
        &Slope { right: 3, down: 1 },
    );
    println!("Encountered {} trees", tree_count)
}

fn count_trees<T>(tree_rows: impl Iterator<Item = T>, slope: &Slope) -> usize
where
    T: AsRef<str>,
{
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
}
