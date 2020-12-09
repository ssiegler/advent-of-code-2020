use crate::puzzle::{Blocks, Puzzle};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeBounds;
use std::str::FromStr;

type Day04 = Blocks<Passport>;

struct Passport(HashMap<String, String>);

impl Passport {
    const REQUIRED_KEYS: &'static [&'static str] = &[
        "byr", // (Birth Year)
        "iyr", // (Issue Year)
        "eyr", // (Expiration Year)
        "hgt", // (Height)
        "hcl", // (Hair Color)
        "ecl", // (Eye Color)
        "pid", // (Passport ID)
               // "cid", // (Country ID)
    ];

    fn has_required_fields(&self) -> bool {
        Self::REQUIRED_KEYS
            .iter()
            .all(|key| self.0.contains_key(*key))
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && self.check_field("pid", is_valid_passport_id)
            && self.check_field("ecl", is_valid_eye_color)
            && self.check_field("hcl", is_valid_hair_color)
            && self.check_field("hgt", is_valid_height)
            && self.check_field("eyr", |year| is_number_in_range(year, 2020..=2030))
            && self.check_field("iyr", |year| is_number_in_range(year, 2010..=2020))
            && self.check_field("byr", |year| is_number_in_range(year, 1920..=2002))
    }

    fn check_field<P>(&self, name: &str, condition: P) -> bool
    where
        P: Fn(&str) -> bool,
    {
        self.0
            .get(name)
            .map(|value| condition(value))
            .unwrap_or(false)
    }
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
fn is_valid_passport_id(pid: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
    }
    RE.is_match(pid)
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
fn is_valid_eye_color(color: &str) -> bool {
    lazy_static! {
        static ref VALID_EYE_COLORS: HashSet<&'static str> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .collect();
    }
    VALID_EYE_COLORS.contains(color)
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn is_valid_hair_color(color: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(color)
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
fn is_valid_height(height: &str) -> bool {
    match height.split_at(height.len() - 2) {
        (cm, "cm") => is_number_in_range(cm, 150..=193),
        (inch, "in") => is_number_in_range(inch, 59..=76),
        _ => false,
    }
}

fn is_number_in_range(input: &str, range: impl RangeBounds<u32>) -> bool {
    input
        .parse::<u32>()
        .map(|number| range.contains(&number))
        .unwrap_or(false)
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Passport(
            input
                .split_whitespace()
                .filter_map(|field| field.splitn(2, ':').map(String::from).collect_tuple())
                .collect(),
        ))
    }
}

impl Puzzle for Day04 {
    fn solve_part1(&self) -> String {
        self.iter()
            .filter(|passport| passport.has_required_fields())
            .count()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.iter()
            .filter(|passport| passport.is_valid())
            .count()
            .to_string()
    }
}

test_puzzle!(Day04;
    Example("\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in", 2, 2),

    File("inputs/day04.txt", 204, 179)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_heigt() {
        assert!(is_valid_height("60in"));
        assert!(is_valid_height("190cm"));
        assert!(!is_valid_height("190in"));
        assert!(!is_valid_height("190"));
    }

    #[test]
    fn validates_hair_color() {
        assert!(is_valid_hair_color("#123abc"));
        assert!(!is_valid_hair_color("#123abz"));
        assert!(!is_valid_hair_color("123abc"));
    }

    #[test]
    fn validates_eye_color() {
        assert!(is_valid_eye_color("brn"));
        assert!(!is_valid_eye_color("wat"));
    }

    #[test]
    fn validates_passport_id() {
        assert!(is_valid_passport_id("000000001"));
        assert!(!is_valid_passport_id("0123456789"));
    }

    #[test]
    fn recognizes_invalid_passports() {
        assert!("\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .parse::<Day04>()
            .expect("Failed to parse passports")
            .iter()
            .all(|passport| !passport.is_valid()));
    }

    #[test]
    fn recognizes_valid_passports() {
        assert!("\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
        .parse::<Day04>()
        .expect("Failed to parse passports")
        .iter()
        .all(|passport| passport.is_valid()));
    }
}
