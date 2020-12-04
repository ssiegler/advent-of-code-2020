use std::collections::HashMap;

fn main() {
    let valid_count =
        read_passports(&std::fs::read_to_string("inputs/day4.txt").expect("Failed to read input"))
            .filter(is_valid)
            .count();
    println!("Got {} valid passports", valid_count);
}

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(input: &'a str) -> Self {
        let fields: HashMap<&str, &str> = input
            .lines()
            .flat_map(|line| line.split_whitespace())
            .filter_map(|pair| {
                let mut pair = pair.splitn(2, ':');
                pair.next().zip(pair.next())
            })
            .collect();
        Passport { fields }
    }
}

fn read_passports(input: &str) -> impl Iterator<Item = Passport> + '_ {
    input.split("\n\n").map(|lines| lines.into())
}

const REQUIRED_KEYS: &[&str] = &[
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
           // "cid", // (Country ID)
];

fn is_valid(passport: &Passport) -> bool {
    REQUIRED_KEYS
        .iter()
        .all(|key| passport.fields.contains_key(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn reports_2_valid_passports_in_example() {
        assert_eq!(2, read_passports(EXAMPLE_INPUT).filter(is_valid).count());
    }
}
