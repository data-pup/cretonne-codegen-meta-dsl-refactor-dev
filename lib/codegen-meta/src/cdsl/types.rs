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
trait ValueType {
    const RUST_NAME: &'static str;

    fn rust_name(&self) -> String;

    /// Return the number of bits in a lane.
    fn lane_bits(&self) -> u64;

    /// Return the number of lanes.
    fn lane_count(&self) -> u64;

    /// Return the total number of bits of an instance of this type.
    fn width(&self) -> u64 {
        self.lane_count() * self.lane_bits()
    }

    /// Return true iff:
    ///     1. self and other have equal number of lanes
    ///     2. each lane in self has at least as many bits as a lane in other
    fn wider_or_equal<T: ValueType>(&self, rhs: &T) -> bool {
        (self.lane_count() == rhs.lane_count()) &&
            (self.lane_bits() >= rhs.lane_bits())
    }
}

/// A concrete scalar type that can appear as a vector lane too.
///
/// Also tracks a unique set of :py:class:`VectorType` instances with this type
/// as the lane type.
trait LaneType
where Self: ValueType,
{}

/// A concrete scalar type that is neither a vector nor a lane type.
///
/// Special types cannot be used to form vectors.
trait SpecialType
where Self: ValueType,
{}

/// A concrete SIMD vector type.
///
/// A vector type has a lane type which is an instance of :class:`LaneType`,
/// and a positive number of lanes.
struct _VectorType {}
// impl ValueType for _VectorType {}

/// A concrete scalar integer type.
pub struct IntType {}
// impl ValueType for IntType {}
// impl LaneType for IntType {}

/// A concrete scalar floating point type.
struct _FloatType {}
// impl ValueType for _FloatType {}
// impl LaneType for _FloatType {}

/// A concrete scalar boolean type.
#[derive(Debug)]
pub struct BoolType {
    bits: u64,
}

impl BoolType {
    /// Initialize a new boolean type with `n` bits.
    pub fn new(bits: u64) -> BoolType {
        BoolType { bits }
    }

    /// Return the number of bits in a lane.
    pub fn lane_bits(&self) -> u64 {
        self.bits
    }

    /// FIXUP: Return the boolean type with the given number
    /// of bits. (Is this aopplicable to the Rust implementation?)
    fn with_bits(bits: u64) -> BoolType {
        unimplemented!();
    }
}

impl ValueType for BoolType {
    const RUST_NAME: &'static str = "BoolType";

    fn rust_name(&self) -> String {
        format!("{}{}", RUST_NAME_PREFIX, Self::RUST_NAME.to_uppercase())
    }

    /// Return the number of bits in a lane.
    fn lane_bits(&self) -> u64 {
        unimplemented!();
    }

    /// Return the number of lanes.
    fn lane_count(&self) -> u64 {
        unimplemented!();
    }

}

impl LaneType for BoolType {}

/// A type representing CPU flags.
///
/// Flags can't be stored in memory.
struct _FlagsType {}
// impl ValueType for _FlagsType {}
// impl SpecialType for _FlagsType {}

/// A flat bitvector type. Used for semantics description only.
struct _BVType {}
// impl ValueType for _BVType {}
