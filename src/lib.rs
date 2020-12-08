mod day01;
mod puzzle;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines(path: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let input = File::open(path).expect("Failed to open input file");
    BufReader::new(input)
        .lines()
        .map(|line| line.expect("Failed to read line"))
}

pub fn read_from_file<T>(path: impl AsRef<Path>) -> impl Iterator<Item = T>
where
    T: FromStr,
{
    read_lines(path)
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(index, line)| {
            line.parse::<T>()
                .unwrap_or_else(|_| panic!("failed to parse line {}: '{:?}'", index, &line))
        })
}

pub fn read_numbers_from_file(path: impl AsRef<Path>) -> Vec<i32> {
    read_from_file(path).collect()
}

#[cfg(test)]
mod tests {
    use crate::read_numbers_from_file;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn reads_numbers_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            "1
2
12
13
{}
1000
-1
{}
",
            i32::max_value(),
            i32::min_value()
        )?;

        assert_eq!(
            read_numbers_from_file(file.path()),
            &[1, 2, 12, 13, i32::max_value(), 1000, -1, i32::min_value()]
        );

        Ok(())
    }
}
