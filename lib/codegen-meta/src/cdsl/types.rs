//! Cretonne ValueType hierarchy

use std::collections::HashMap;

use base::types as base_types;

// Numbering scheme for value types:
//
// 0: Void
// 0x01-0x6f: Special types
// 0x70-0x7f: Lane types
// 0x80-0xff: Vector types
//
// Vector types are encoded with the lane type in the low 4 bits and log2(lanes)
// in the high 4 bits, giving a range of 2-256 lanes.
static LANE_BASE: u16 = 0x70;

static RUST_NAME_PREFIX: &'static str = "ir::types::";

// ValueType instances (i8, i32, ...) are provided in the `base.types` module.

/// A concrete SSA value type.
///
/// All SSA values have a type that is described by an instance of `ValueType`
/// or one of its subclasses.
pub enum ValueType {
    Lane(LaneType),
}

impl ValueType {
    pub fn rust_name(&self) -> String {
        match self {
            _ => unimplemented!(),
        }
    }
}

/// The kinds of elements in a lane.
pub enum LaneType {
    BoolType(Boolean),
}

impl LaneType
{
    /// Return the number of bits in a lane.
    fn lane_bits(&self) -> u64 {
        match self {
            LaneType::BoolType(b) => b.lane_bits(),
        }
    }

    /// Return the number of bits in a lane.
    fn lane_count(&self) -> u64 {
        1
    }

    /// Return the total number of bits of an instance of this type.
    fn width(&self) -> u64 {
        self.lane_count() * self.lane_bits()
    }

    /// Return true iff:
    ///     1. self and other have equal number of lanes
    ///     2. each lane in self has at least as many bits as a lane in other
    fn wider_or_equal(&self, rhs: &LaneType) -> bool {
        (self.lane_count() == rhs.lane_count()) &&
            (self.lane_bits() >= rhs.lane_bits())
    }
}

/// A concrete scalar boolean type.
#[derive(Debug)]
pub struct Boolean {
    bits: u64,
}

impl Boolean {
    const RUST_NAME: &'static str = "BoolType";

    /// Initialize a new boolean type with `n` bits.
    pub fn new(bits: u64) -> Boolean {
        Boolean { bits }
    }

    /// Create a Boolean object with the given number of bits.
    pub fn with_bits(bits: u64) -> Boolean {
        unimplemented!();
    }

    /// Get the name of the type.
    fn rust_name() -> String {
        format!("{}{}", RUST_NAME_PREFIX, Self::RUST_NAME.to_uppercase())
    }

    /// Get the number of bits in a lane.
    fn lane_bits(&self) -> u64 {
        self.bits
    }
}

// /// A type representing CPU flags.
// ///
// /// Flags can't be stored in memory.
// struct _FlagsType {}

// /// A flat bitvector type. Used for semantics description only.
// struct _BVType {}

// /// A concrete scalar type that is neither a vector nor a lane type.
// ///
// /// Special types cannot be used to form vectors.
// pub struct SpecialType {}

// /// A concrete SIMD vector type.
// ///
// /// A vector type has a lane type which is an instance of :class:`LaneType`,
// /// and a positive number of lanes.
// struct _VectorType {}

// /// A concrete scalar integer type.
// struct _IntType {}

// /// A concrete scalar floating point type.
// struct _FloatType {}
