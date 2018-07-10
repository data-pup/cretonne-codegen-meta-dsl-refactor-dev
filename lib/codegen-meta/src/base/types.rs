//! This module predefines all the Cretonne scalar types.

// DEVELOPMENT NOTE: This might require further revision, this defines the
// sizes of int and bool scalar types, rather than the types themselves.

// FIXUP: Some of the name/doc logic may be able to be cleaned up using to_string
// or other various traits.

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
    /// Get the name of a boolean variant.
    pub fn name(&self) -> &str {
        match self {
            Bool::B1 => "B1",
            Bool::B8 => "B8",
            Bool::B16 => "B16",
            Bool::B32 => "B32",
            Bool::B64 => "B64",
        }
    }

    /// Get the documentation comment of a boolean variant.
    pub fn doc(&self) -> &str {
        match self {
            Bool::B1 => "A boolean type with 1 bits.",
            Bool::B8 => "A boolean type with 8 bits.",
            Bool::B16 => "A boolean type with 16 bits.",
            Bool::B32 => "A boolean type with 32 bits.",
            Bool::B64 => "A boolean type with 64 bits.",
        }
    }

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

    /// Get the boolean variant that corresponds to a given number.
    pub fn _by_number(_i: u8) -> Option<Flag> {
        unimplemented!()
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

struct _IntIterator {
    index: usize,
}

impl _IntIterator {
    fn _new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for _IntIterator {
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

    /// Get the documentation comment of a flag variant.
    pub fn doc(&self) -> &str {
        match self {
            Flag::IFlags => "CPU flags representing the result of an integer comparison. These flags can be tested with an :type:`intcc` condition code.
            ",
            Flag::FFlags => "CPU flags representing the result of a floating point comparison. These flags can be tested with a :type:`floatcc` condition code.
            ",
        }
    }

    /// Get the number of a flag variant.
    pub fn number(&self) -> u8 {
        match self {
            Flag::IFlags => 1,
            Flag::FFlags => 2,
        }
    }

    /// Get the flag variant that corresponds to a given number.
    pub fn _by_number(i: u8) -> Option<Flag> {
        match i {
            0 => Some(Flag::IFlags),
            1 => Some(Flag::FFlags),
            _ => None,
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
