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

pub struct LaneType {
    bits: u64,
    tag: LaneTypeTag,
}

impl LaneType {
    /// Create a lane of the type with the given number of bits.
    pub fn new<T>(bits: u64, t: T) -> LaneType
    where
        T: Into<LaneTypeTag>
    {
        unimplemented!();
    }

    /// Get the name of the type.
    fn rust_name(&self) -> String {
        let type_name: &'static str = match self.tag {
            LaneTypeTag::BoolType(_) => "BoolType",
            LaneTypeTag::IntType(_) => "IntType",
            LaneTypeTag::FloatType(_) => "FloatType",
        };

        format!("{}{}", RUST_NAME_PREFIX, type_name.to_uppercase())
    }

    /// Return the number of bits in a lane.
    fn lane_count(&self) -> u64 {
        1
    }

    /// Return the number of bits in a lane.
    pub fn lane_bits(&self) -> u64 {
        self.bits
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

/// The kinds of elements in a lane.
pub enum LaneTypeTag {
    BoolType(Boolean),
    IntType(Integer),
    FloatType(FloatingPoint),
}

/// A concrete scalar boolean type.
#[derive(Debug)]
pub struct Boolean;

impl Boolean {
    /// Initialize a new boolean type with `n` bits.
    pub fn new() -> Boolean {
        Boolean
    }
}

/// A concrete scalar integer type.
struct Integer;

impl Integer {
    /// Initialize a new integer type with `n` bits.
    pub fn new() -> Integer {
        Integer
    }
}

/// A concrete scalar floating point type.
struct FloatingPoint;

impl FloatingPoint {
    /// Initialize a new integer type with `n` bits.
    pub fn new() -> FloatingPoint {
        FloatingPoint
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
