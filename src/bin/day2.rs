use std::str::FromStr;

use anyhow::anyhow;
use anyhow::{Context, Result};

fn main() {}

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
                password: "cdefg".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn validates_passwords() {
        let passwords: Vec<Password> = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            .lines()
            .map(|line| line.parse::<Password>())
            .collect::<Result<Vec<Password>, _>>()
            .expect("");

        assert_eq!(
            passwords
                .iter()
                .filter(|password| password.is_valid())
                .count(),
            2
        )
    }
}
