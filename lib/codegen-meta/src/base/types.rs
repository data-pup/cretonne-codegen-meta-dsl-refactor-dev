//! This module predefines all the Cretonne scalar types.

// DEVELOPMENT NOTE: This might require further revision, this defines the
// sizes of int and bool scalar types, rather than the types themselves.

pub enum _Bools {
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

struct IntIterator { index: usize }

impl IntIterator {
    fn new() -> Self {
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
            _ => None
        };
        self.index += 1;
        res
    }
}

pub enum Flag {
    /// CPU flags representing the result of an integer comparison. These flags
    /// can be tested with an :type:`intcc` condition code.
    IFlag,
    /// CPU flags representing the result of a floating point comparison. These
    /// flags can be tested with a :type:`floatcc` condition code.
    FFlag,
}

pub struct FlagIterator { index: usize }

impl FlagIterator {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for FlagIterator {
    type Item = Flag;
    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.index {
            0 => Some(Flag::IFlag),
            1 => Some(Flag::FFlag),
            _ => None,
        };
        self.index += 1;
        res
    }
}
