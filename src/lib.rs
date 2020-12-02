use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_from_file<T, F, E>(path: &Path, parse: F) -> Vec<T>
where
    F: Fn(&str) -> Result<T, E>,
{
    let input = File::open(path).expect("Failed to open input file");
    BufReader::new(input)
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(index, line)| {
            parse(&line).unwrap_or_else(|_| panic!("failed to parse line {}: '{:?}'", index, &line))
        })
        .collect()
}

pub fn read_numbers_from_file(path: &Path) -> Vec<i32> {
    read_from_file(path, |str| str.parse())
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
