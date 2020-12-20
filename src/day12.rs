use crate::{read_lines, ParseError};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Orientation {
    East,
    South,
    West,
    North,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    Move(Option<Orientation>, usize),
    TurnLeft,
    TurnRight,
    TurnAround,
}

impl Instruction {
    fn move_to(orientation: Orientation, value: usize) -> Self {
        Self::Move(Some(orientation), value)
    }

    fn forward(value: usize) -> Self {
        Self::Move(None, value)
    }

    fn turn_left(value: usize) -> Result<Self, ParseError> {
        Ok(match value {
            90 => Self::TurnLeft,
            180 => Self::TurnAround,
            270 => Self::TurnRight,
            _ => return Err(ParseError::FormatError),
        })
    }

    fn turn_right(value: usize) -> Result<Self, ParseError> {
        Ok(match value {
            90 => Self::TurnRight,
            180 => Self::TurnAround,
            270 => Self::TurnLeft,
            _ => return Err(ParseError::FormatError),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Position {
    orientation: Orientation,
    north: isize,
    east: isize,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            orientation: Orientation::East,
            north: 0,
            east: 0,
        }
    }
}

impl Position {
    fn execute_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(orientation, value) => {
                let orientation = orientation.clone().unwrap_or(self.orientation.clone());
                self.move_to(&orientation, value)
            }
            Instruction::TurnAround => self.turn_around(),
            Instruction::TurnLeft => self.turn_left(),
            Instruction::TurnRight => self.turn_right(),
        }
    }

    fn move_to(&mut self, orientation: &Orientation, value: &usize) {
        match orientation {
            Orientation::East => self.east += *value as isize,
            Orientation::South => self.north -= *value as isize,
            Orientation::West => self.east -= *value as isize,
            Orientation::North => self.north += *value as isize,
        }
    }

    fn turn_around(&mut self) {
        self.orientation = match self.orientation {
            Orientation::East => Orientation::West,
            Orientation::South => Orientation::North,
            Orientation::West => Orientation::East,
            Orientation::North => Orientation::South,
        }
    }
    fn turn_right(&mut self) {
        self.orientation = match self.orientation {
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
            Orientation::North => Orientation::East,
        }
    }
    fn turn_left(&mut self) {
        self.orientation = match self.orientation {
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
            Orientation::North => Orientation::West,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (action, value) = input.split_at(1);
        Ok(match (action, value.parse::<usize>()?) {
            ("N", value) => Instruction::move_to(Orientation::North, value),
            ("E", value) => Instruction::move_to(Orientation::East, value),
            ("S", value) => Instruction::move_to(Orientation::South, value),
            ("W", value) => Instruction::move_to(Orientation::West, value),
            ("F", value) => Instruction::forward(value),
            ("R", value) => Instruction::turn_right(value)?,
            ("L", value) => Instruction::turn_left(value)?,
            _ => return Err(ParseError::FormatError),
        })
    }
}

#[aoc_generator(day12)]
fn read_instructions(input: &str) -> Result<Vec<Instruction>, ParseError> {
    read_lines(input)
}

#[aoc(day12, part1)]
fn measure_manhattan_distance(instructions: &[Instruction]) -> usize {
    let mut position = Position::default();
    position.execute_instructions(instructions);
    position.north.abs() as usize + position.east.abs() as usize
}

#[aoc(day12, part2)]
fn measure_manhattan_distance_using_waypoint(instructions: &[Instruction]) -> usize {
    let mut ship = Ship::default();
    ship.execute_instructions(instructions);
    ship.position.0.abs() as usize + ship.position.1.abs() as usize
}

#[derive(Debug, Eq, PartialEq)]
struct Ship {
    waypoint: (isize, isize),
    position: (isize, isize),
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            waypoint: (10, 1),
            position: (0, 0),
        }
    }
}

impl Ship {
    fn execute_instructions(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(None, times) => {
                self.position = (
                    *times as isize * self.waypoint.0 + self.position.0,
                    *times as isize * self.waypoint.1 + self.position.1,
                )
            }
            Instruction::Move(Some(direction), amount) => match direction {
                Orientation::East => self.waypoint.0 += *amount as isize,
                Orientation::South => self.waypoint.1 -= *amount as isize,
                Orientation::West => self.waypoint.0 -= *amount as isize,
                Orientation::North => self.waypoint.1 += *amount as isize,
            },
            Instruction::TurnLeft => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            Instruction::TurnRight => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            Instruction::TurnAround => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref EXAMPLE: Vec<Instruction> = read_instructions(
            "\
F10
N3
F7
R90
F11",
        )
        .expect("Failed to read example");
    }

    #[test]
    fn moves_correctly_for_example() {
        let mut position = Position::default();
        position.execute_instructions(&EXAMPLE);
        assert_eq!(
            position,
            Position {
                orientation: Orientation::South,
                north: -8,
                east: 17
            }
        );
    }

    #[test]
    fn solves_example() {
        assert_eq!(measure_manhattan_distance(&EXAMPLE), 25);
    }

    lazy_static! {
        static ref INPUT: Vec<Instruction> =
            read_instructions(include_str!("../input/2020/day12.txt"))
                .expect("Failed to read input");
    }

    #[test]
    fn solves_part1() {
        assert_eq!(measure_manhattan_distance(&INPUT), 1152);
    }

    #[test]
    fn moves_correctly_using_waypoint_for_example() {
        let mut ship = Ship::default();
        ship.execute_instructions(&EXAMPLE);
        assert_eq!(
            ship,
            Ship {
                position: (214, -72),
                waypoint: (4, -10)
            }
        );
    }

    #[test]
    fn solves_example_part2() {
        assert_eq!(measure_manhattan_distance_using_waypoint(&EXAMPLE), 286);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(measure_manhattan_distance_using_waypoint(&INPUT), 58637);
    }
}
