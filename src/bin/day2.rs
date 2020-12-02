use std::path::Path;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::{Context, Result};

use advent_of_code::read_from_file;

fn main() {
    let mut valid_count = 0;
    let mut officially_valid_count = 0;
    for password in read_from_file::<Password>(Path::new("inputs/day2.txt")) {
        if password.is_valid() {
            valid_count += 1;
        }
        if password.is_officially_valid() {
            officially_valid_count += 1;
        }
    }
    println!("Got {} valid passwords", valid_count);
    println!("Got {} officially valid passwords", officially_valid_count);
}

#[derive(Debug, Eq, PartialEq)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|ch| *ch == self.letter)
            .count();
        count >= self.min && count <= self.max
    }

    fn is_officially_valid(&self) -> bool {
        match (
            self.password
                .chars()
                .nth(self.min - 1)
                .map(|ch| ch == self.letter),
            self.password
                .chars()
                .nth(self.max - 1)
                .map(|ch| ch == self.letter),
        ) {
            (Some(true), Some(false)) => true,
            (Some(false), Some(true)) => true,
            _ => false,
        }
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            ^(?P<min>[\d]+)
            -
            (?P<max>[\d]+)
            \s
            (?P<letter>[\w])
            :\s
            (?P<password>[\w]+)$
            "
            )
            .unwrap();
        }
        let cap = RE
            .captures(input)
            .ok_or_else(|| anyhow!("Password syntax mismatch: '{}'", input))?;
        let (min, max, letter, password) = (
            cap.name("min")
                .ok_or_else(|| anyhow!("Missing 'min'"))?
                .as_str()
                .parse::<usize>()
                .with_context(|| format!("Parsing 'min' in '{}'", input))?,
            cap.name("max")
                .ok_or_else(|| anyhow!("Missing 'max'"))?
                .as_str()
                .parse::<usize>()
                .with_context(|| format!("Parsing 'max' in '{}'", input))?,
            cap.name("letter")
                .and_then(|letter| letter.as_str().chars().next())
                .ok_or_else(|| anyhow!("Missing 'letter'"))?,
            cap.name("password")
                .ok_or_else(|| anyhow!("Missing 'password'"))?
                .as_str()
                .to_string(),
        );
        Ok(Password {
            min,
            max,
            letter,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password() -> Result<()> {
        assert_eq!(
            "1-3 b: cdefg".parse::<Password>()?,
            Password {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            }
        );
        Ok(())
    }

    fn example_input() -> Vec<Password> {
        vec![
            Password {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            Password {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            Password {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ]
    }

    #[test]
    fn validates_passwords() {
        assert_eq!(
            example_input()
                .iter()
                .map(|password| password.is_valid())
                .collect::<Vec<_>>(),
            &[true, false, true]
        )
    }

    #[test]
    fn validates_passwords_officially() {
        assert_eq!(
            example_input()
                .iter()
                .map(|password| password.is_officially_valid())
                .collect::<Vec<_>>(),
            &[true, false, false]
        );
    }
}
