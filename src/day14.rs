use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

use bit_vec::{BitBlock, BitVec};
use itertools::Itertools;

use crate::day14::Instruction::{UpdateMask, Write};
use crate::{read_lines, ParseError};

const BITS: usize = 36;

#[derive(Debug, PartialEq, Clone)]
struct Mask {
    ones: BitVec,
    zeroes: BitVec,
}

impl FromStr for Mask {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() != BITS {
            return Err(ParseError::FormatError);
        }
        let zeroes: BitVec = input.bytes().map(|bit| bit == b'0').collect();
        debug_assert!(zeroes.len() == BITS);
        let ones: BitVec = input.bytes().map(|bit| bit == b'1').collect();
        debug_assert!(ones.len() == BITS);
        Ok(Mask { ones, zeroes })
    }
}

impl Mask {
    fn apply_to(&self, value: &Value) -> Value {
        let mut value = value.clone();
        value.0.or(&self.ones);
        value.0.difference(&self.zeroes);
        value
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Value(BitVec);

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value(
            BitVec::from_bytes(&value.to_be_bytes())
                .iter()
                .skip(u64::bits() - BITS)
                .collect(),
        )
    }
}

impl From<&Value> for u64 {
    fn from(value: &Value) -> Self {
        let mut u64_bits = BitVec::with_capacity(u64::bits());
        u64_bits.grow(u64::bits() - BITS, false);
        u64_bits.extend(&value.0);
        u64::from_be_bytes(u64_bits.to_bytes().try_into().unwrap())
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input.parse::<u64>()?.into())
    }
}

type Memory = HashMap<Value, Value>;

struct Computer {
    mask: Mask,
    memory: Memory,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    UpdateMask(Mask),
    Write { value: Value, address: Value },
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with("mask = ") {
            return Ok(UpdateMask(input["mask = ".len()..].parse::<Mask>()?));
        } else if input.starts_with("mem") {
            if let Some(("mem", address, value)) = input.splitn(3, &['[', ']'][..]).collect_tuple()
            {
                return Ok(Write {
                    value: value[" = ".len()..].parse::<Value>()?,
                    address: address.parse::<Value>()?,
                });
            }
        }
        Err(ParseError::FormatError)
    }
}

impl Computer {
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            UpdateMask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::Write { value, address } => {
                self.memory
                    .insert(address.clone(), self.mask.apply_to(value));
            }
        }
    }

    fn execute_program(&mut self, program: &[Instruction]) {
        for instruction in program {
            self.execute_instruction(instruction)
        }
    }

    fn sum_memory(&self) -> u64 {
        self.memory
            .values()
            .map(|value| u64::from(value))
            .sum::<u64>()
    }
}

impl Default for Computer {
    fn default() -> Self {
        Computer {
            mask: Mask {
                ones: BitVec::from_elem(BITS, false),
                zeroes: BitVec::from_elem(BITS, false),
            },
            memory: Default::default(),
        }
    }
}

#[aoc_generator(day14)]
fn read_program(input: &str) -> Result<Vec<Instruction>, ParseError> {
    read_lines(input)
}

#[aoc(day14, part1)]
fn execute_program(program: &[Instruction]) -> u64 {
    let mut computer = Computer::default();
    computer.execute_program(&program);
    computer.sum_memory()
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn applies_mask_to_values() {
        let mask =
            Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").expect("failed to parse mask");
        assert_eq!(mask.apply_to(&11.into()), 73.into());
        assert_eq!(mask.apply_to(&0.into()), 64.into());
    }

    #[test]
    fn value_is_unaffected_by_mask() {
        let mask =
            Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").expect("failed to parse mask");
        assert_eq!(mask.apply_to(&101.into()), 101.into());
    }

    #[test]
    fn parses_mask_instruction() {
        let mut ones = BitVec::from_elem(36, false);
        ones.set(29, true);
        let mut zeroes = BitVec::from_elem(36, false);
        zeroes.set(34, true);
        assert_eq!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse::<Instruction>(),
            Ok(Instruction::UpdateMask(Mask { ones, zeroes }))
        );
    }

    #[test]
    fn parses_mem_instruction() {
        assert_eq!(
            "mem[8] = 11".parse::<Instruction>(),
            Ok(Instruction::Write {
                value: 11.into(),
                address: 8.into(),
            })
        );
    }

    #[test]
    fn convert_values_back_and_forth() {
        assert_eq!(u64::from(&Value::from(0x0EFFFFFFFF)), 0x0EFFFFFFFF);
    }

    #[test]
    fn solve_example_part1() {
        assert_eq!(
            execute_program(
                &read_program(
                    "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
                )
                .expect("failed to parse example")
            ),
            165
        );
    }

    const INPUT: &str = include_str!("../input/2020/day14.txt");

    #[test]
    fn solve_part1() {
        assert_eq!(
            execute_program(&read_program(INPUT).expect("failed to read input")),
            11884151942312
        );
    }
}