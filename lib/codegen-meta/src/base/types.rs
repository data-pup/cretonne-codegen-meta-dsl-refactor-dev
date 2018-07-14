//! This module predefines all the Cretonne scalar types.

use std::fmt;

// Numbering scheme for value types:
//
// 0: Void
// 0x01-0x6f: Special types
// 0x70-0x7f: Lane types
// 0x80-0xff: Vector types
//
// Vector types are encoded with the lane type in the low 4 bits and log2(lanes)
// in the high 4 bits, giving a range of 2-256 lanes.
static _LANE_BASE: u8 = 0x70;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Bool {
    /// 1-bit bool.
    B1 = 1,
    /// 8-bit bool.
    B8 = 8,
    /// 16-bit bool.
    B16 = 16,
    /// 32-bit bool.
    B32 = 32,
    /// 64-bit bool.
    B64 = 64,
}

impl Bool {
    /// Get the number of a boolean variant.
    pub fn number(&self) -> u8 {
        let offset = match self {
            Bool::B1 => 0,
            Bool::B8 => 1,
            Bool::B16 => 2,
            Bool::B32 => 3,
            Bool::B64 => 4,
        };

        _LANE_BASE + offset
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Bool::B1 => "b1",
            Bool::B8 => "b8",
            Bool::B16 => "b16",
            Bool::B32 => "b32",
            Bool::B64 => "b64",
        })
    }
}

pub struct BoolIterator {
    index: usize,
}

impl BoolIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for BoolIterator {
    type Item = Bool;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Bool::B1),
            1 => Some(Bool::B8),
            2 => Some(Bool::B16),
            3 => Some(Bool::B32),
            4 => Some(Bool::B64),
            _ => None,
        };
        self.index += 1;
        res
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Int {
    /// 8-bit int.
    I8 = 8,
    /// 16-bit int.
    I16 = 16,
    /// 32-bit int.
    I32 = 32,
    /// 64-bit int.
    I64 = 64,
}

impl Int {
    /// Get the number of a integer variant.
    pub fn number(&self) -> u8 {
        let offset = 5 + match self {
            Int::I8 => 0,
            Int::I16 => 1,
            Int::I32 => 2,
            Int::I64 => 3,
        };

        _LANE_BASE + offset
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Int::I8 => "i8",
            Int::I16 => "i16",
            Int::I32 => "i32",
            Int::I64 => "i64",
        })
    }
}

pub struct IntIterator {
    index: usize,
}

impl IntIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for IntIterator {
    type Item = Int;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Int::I8),
            1 => Some(Int::I16),
            2 => Some(Int::I32),
            3 => Some(Int::I64),
            _ => None,
        };
        self.index += 1;
        res
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Float {
    F32 = 32,
    F64 = 64,
}

impl Float {
    /// Get the number of a flag variant.
    pub fn number(&self) -> u8 {
        let offset = 9 + match self {
            Float::F32 => 0,
            Float::F64 => 1,
        };

        _LANE_BASE + offset
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Float::F32 => "f32",
            Float::F64 => "f64",
        })
    }
}

/// Iterator through the variants of the Float enum.
pub struct FloatIterator {
    index: usize,
}

impl FloatIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for FloatIterator {
    type Item = Float;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Float::F32),
            1 => Some(Float::F64),
            _ => None,
        };
        self.index += 1;
        res
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Flag {
    /// CPU flags from an integer comparison.
    IFlags,
    /// CPU flags from a floating point comparison.
    FFlags,
}

impl Flag {
    /// Get the name of a flag variant.
    pub fn name(&self) -> &str {
        match self {
            Flag::IFlags => "iflags",
            Flag::FFlags => "fflags",
        }
    }

    /// Get the number of a flag variant.
    pub fn number(&self) -> u8 {
        match self {
            Flag::IFlags => 1,
            Flag::FFlags => 2,
        }
    }
}

/// Iterator through the variants of the Flag enum.
pub struct FlagIterator {
    index: usize,
}

impl FlagIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for FlagIterator {
    type Item = Flag;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Flag::IFlags),
            1 => Some(Flag::FFlags),
            _ => None,
        };
        self.index += 1;
        res
    }
}

#[cfg(test)]
mod iter_tests {
    use super::*;

    #[test]
    fn bool_iter_works() {
        let mut bool_iter = BoolIterator::new();
        assert_eq!(bool_iter.next(), Some(Bool::B1));
        assert_eq!(bool_iter.next(), Some(Bool::B8));
        assert_eq!(bool_iter.next(), Some(Bool::B16));
        assert_eq!(bool_iter.next(), Some(Bool::B32));
        assert_eq!(bool_iter.next(), Some(Bool::B64));
        assert_eq!(bool_iter.next(), None);
    }

    #[test]
    fn int_iter_works() {
        let mut int_iter = IntIterator::new();
        assert_eq!(int_iter.next(), Some(Int::I8));
        assert_eq!(int_iter.next(), Some(Int::I16));
        assert_eq!(int_iter.next(), Some(Int::I32));
        assert_eq!(int_iter.next(), Some(Int::I64));
        assert_eq!(int_iter.next(), None);
    }

    #[test]
    fn float_iter_works() {
        let mut float_iter = FloatIterator::new();
        assert_eq!(float_iter.next(), Some(Float::F32));
        assert_eq!(float_iter.next(), Some(Float::F64));
        assert_eq!(float_iter.next(), None);
    }

    #[test]
    fn flag_iter_works() {
        let mut flag_iter = FlagIterator::new();
        assert_eq!(flag_iter.next(), Some(Flag::IFlags));
        assert_eq!(flag_iter.next(), Some(Flag::FFlags));
        assert_eq!(flag_iter.next(), None);
    }
}
