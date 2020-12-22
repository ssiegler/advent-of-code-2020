use std::str::FromStr;

use bit_vec::{BitBlock, BitVec};

use crate::ParseError;
use std::ops::Deref;

struct Mask {
    ones: BitVec,
    zeroes: BitVec,
}

#[derive(Debug, Eq, PartialEq)]
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

impl Deref for Value {
    type Target = BitVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const BITS: usize = 36;

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
    fn apply_to(&self, value: Value) -> Value {
        let mut value = value.clone();
        value.or(&self.ones);
        value.difference(&self.zeroes);
        Value(value)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn applies_mask_to_values() {
        let mask =
            Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").expect("failed to parse mask");
        assert_eq!(mask.apply_to(11.into()), 73.into());
        assert_eq!(mask.apply_to(0.into()), 64.into());
    }

    #[test]
    fn value_is_unaffected_by_mask() {
        let mask =
            Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").expect("failed to parse mask");
        assert_eq!(mask.apply_to(101.into()), 101.into());
    }
}
