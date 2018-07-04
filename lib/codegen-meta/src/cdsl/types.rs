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
    BV(BVType),
    Lane(LaneType),
    Special(SpecialType),
    Vector(VectorType),
}

impl ValueType {
    pub fn all_special_types() -> SpecialTypeIterator {
        SpecialTypeIterator::new()
    }

    pub fn rust_name(&self) -> String {
        match self {
            ValueType::Lane(l) => l.rust_name(),
            ValueType::Special(s) => s.rust_name(),
            _ => unimplemented!(),
        }
    }

    pub fn doc(&self) -> String {
        match self {
            ValueType::Special(s) => s.doc(),
            _ => unimplemented!(),
        }
    }
}


/// A concrete scalar type that can appear as a vector lane too.
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
    /// Initialize a new floating point type with `n` bits.
    pub fn new() -> FloatingPoint {
        FloatingPoint
    }
}

/// A concrete SIMD vector type.
///
/// A vector type has a lane type which is an instance of `LaneType`,
/// and a positive number of lanes.
struct VectorType {
    base: LaneType,
    lanes: u8,
}

impl VectorType {
    /// Initialize a new integer type with `n` bits.
    pub fn new(base: LaneType, lanes: u8) -> VectorType {
        VectorType { base, lanes }
    }

    /// Return the number of lanes.
    pub fn lane_count(&self) -> u8 {
        self.lanes
    }

    /// Return the number of bits in a lane.
    pub fn lane_bits(&self) -> u64 {
        self.base.lane_bits()
    }
}

/// A flat bitvector type. Used for semantics description only.
struct BVType;

impl BVType {
    /// Initialize a new integer type with `n` bits.
    pub fn new() -> BVType {
        BVType
    }
}

// /// A type representing CPU flags.
// ///
// /// Flags can't be stored in memory.
// struct _FlagsType {}

// /// A flat bitvector type. Used for semantics description only.
// struct _BVType {}

/// A concrete scalar type that is neither a vector nor a lane type.
///
/// Special types cannot be used to form vectors.
pub struct SpecialType {
    tag: SpecialTypeTag,
}

impl SpecialType {
    pub fn rust_name(&self) -> String {
        let type_name: &'static str = match self.tag {
            SpecialTypeTag::Flag(_) => "FlagType",
            _ => unimplemented!(),
        };

        format!("{}{}", RUST_NAME_PREFIX, type_name.to_uppercase())
    }

    pub fn doc(&self) -> String {
        unimplemented!();
    }
}

pub enum SpecialTypeTag {
    Flag(base_types::Flag),
}

pub struct SpecialTypeIterator {
    flag_iter: base_types::FlagIterator,
}

impl SpecialTypeIterator {
    fn new() -> Self {
        Self { flag_iter: base_types::FlagIterator::new() }
    }
}

impl Iterator for SpecialTypeIterator {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(flag) = self.flag_iter.next() {
            let next_item = SpecialType { tag: SpecialTypeTag::Flag(flag) };
            Some(ValueType::Special(next_item))
        } else {
            None
        }
    }
}

// /// A concrete SIMD vector type.
// ///
// /// A vector type has a lane type which is an instance of :class:`LaneType`,
// /// and a positive number of lanes.
// struct _VectorType {}
