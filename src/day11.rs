use crate::ParseError;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

struct Seats {
    columns: usize,
    tiles: Vec<u8>,
}

impl FromStr for Seats {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut columns = None;
        let mut tiles = Vec::new();
        for line in input.lines() {
            if *columns.get_or_insert(line.len()) != line.len() {
                return Err(ParseError::FormatError);
            }
            tiles.extend(line.as_bytes());
        }
        Ok(Seats {
            columns: columns.unwrap_or(0),
            tiles,
        })
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for item in self
            .tiles
            .chunks(self.columns)
            .map(String::from_utf8_lossy)
            .intersperse("\n".into())
        {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod should {
    use super::*;

    const EXAMPLE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn parse_and_display_seats() {
        assert_eq!(
            Seats::from_str(EXAMPLE)
                .expect("Failed to parse example")
                .to_string(),
            EXAMPLE
        );
    }
}
