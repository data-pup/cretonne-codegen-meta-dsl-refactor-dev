//! Cretonne ValueType hierarchy

// Temporary disabled: Unused at the moment.
// use std::collections::HashMap;

use base::types as base_types;

static _RUST_NAME_PREFIX: &'static str = "ir::types::";

// ValueType instances (i8, i32, ...) are provided in the `base.types` module.

/// A concrete SSA value type.
///
/// All SSA values have a type that is described by an instance of `ValueType`
/// or one of its subclasses.
pub enum ValueType {
    _BV(_BVType),
    Lane(LaneType),
    Special(SpecialType),
    _Vector(VectorType),
}

impl ValueType {
    /// Iterate through all of the special types (neither lanes nor vectors).
    pub fn all_special_types() -> SpecialTypeIterator {
        SpecialTypeIterator::new()
    }

    /// Iterate through all of the lane types.
    pub fn all_lane_types() -> LaneTypeIterator {
        LaneTypeIterator::new()
    }

    /// Get the name of this type.
    pub fn name(&self) -> &str {
        match self {
            ValueType::Lane(l) => l.name(),
            ValueType::Special(s) => s.name(),
            _ => unimplemented!(),
        }
    }

    /// Return a string containing the documentation comment for this type.
    pub fn doc(&self) -> String {
        match self {
            ValueType::Lane(l) => l.doc(),
            ValueType::Special(s) => s.doc(),
            _ => unimplemented!(),
        }
    }

    /// Find the unique number associated with this type.
    pub fn number(&self) -> u8 {
        match self {
            ValueType::Lane(l) => l.number(),
            ValueType::Special(s) => s.number(),
            _ => unimplemented!(),
        }
    }

    /// Find the number of bytes that this type occupies in memory.
    pub fn membytes(&self) -> u64 {
        match self {
            _ => unimplemented!(),
        }
    }

    pub fn _rust_name(&self) -> String {
        match self {
            ValueType::Lane(l) => l._rust_name(),
            ValueType::Special(s) => s._rust_name(),
            _ => unimplemented!(),
        }
    }
}

/// A concrete scalar type that can appear as a vector lane too.
pub struct LaneType {
    _bits: u64,
    _tag: LaneTypeTag,
}

impl LaneType {
    /// Create a lane of the type with the given number of bits.
    pub fn _new<T>(_bits: u64, _t: T) -> LaneType
    where
        T: Into<LaneTypeTag>,
    {
        unimplemented!();
    }

    /// Get the name of this type.
    pub fn name(&self) -> &str {
        self._tag.name()
    }

    pub fn doc(&self) -> String {
        match self._tag {
            LaneTypeTag::_BoolType(_) => format!("A boolean type with {} bits.", self._bits),
            LaneTypeTag::_IntType(_) => format!("An integer type with {} bits.", self._bits),
            LaneTypeTag::_FloatType(base_types::Float::F32) => String::from("A 32-bit floating point type represented in the IEEE 754-2008 *binary32* interchange format. This corresponds to the :c:type:`float` type in most C implementations."),
            LaneTypeTag::_FloatType(base_types::Float::F64) => String::from("A 64-bit floating point type represented in the IEEE 754-2008 *binary64* interchange format. This corresponds to the :c:type:`double` type in most C implementations.")
        }
    }

    pub fn number(&self) -> u8 {
        self._tag.number()
    }

    /// Get a vector type with this type as the lane type.
    ///
    /// For example, ``i32.by(4)`` returns the :obj:`i32x4` type.
    pub fn _by(&self, _lanes: u64) -> VectorType {
        unimplemented!();
    }

    /// Return the number of bits in a lane.
    fn _lane_count(&self) -> u64 {
        1
    }

    /// Return the number of bits in a lane.
    pub fn _lane_bits(&self) -> u64 {
        self._bits
    }

    /// Return the total number of bits of an instance of this type.
    fn _width(&self) -> u64 {
        self._lane_count() * self._lane_bits()
    }

    /// Return true iff:
    ///     1. self and other have equal number of lanes
    ///     2. each lane in self has at least as many bits as a lane in other
    fn _wider_or_equal(&self, rhs: &LaneType) -> bool {
        (self._lane_count() == rhs._lane_count()) && (self._lane_bits() >= rhs._lane_bits())
    }
    /// Get the name of the type.
    /// FIXUP: Describe the distinction between this method and `name`
    fn _rust_name(&self) -> String {
        let type_name: &'static str = match self._tag {
            LaneTypeTag::_BoolType(_) => "BoolType",
            LaneTypeTag::_IntType(_) => "IntType",
            LaneTypeTag::_FloatType(_) => "FloatType",
        };

        format!("{}{}", _RUST_NAME_PREFIX, type_name.to_uppercase())
    }
}


/// The kinds of elements in a lane.
pub enum LaneTypeTag {
    _BoolType(base_types::Bool),
    _IntType(base_types::Int),
    _FloatType(base_types::Float),
}

impl LaneTypeTag {
    fn name(&self) -> &str {
        match self {
            LaneTypeTag::_BoolType(b) => b.name(),
            LaneTypeTag::_IntType(i) => i.name(),
            LaneTypeTag::_FloatType(f) => f.name(),
        }
    }

    fn number(&self) -> u8 {
        match self {
            LaneTypeTag::_BoolType(b) => b.number(),
            LaneTypeTag::_IntType(i) => i.number(),
            LaneTypeTag::_FloatType(f) => f.number(),
        }
    }
}

pub struct LaneTypeIterator {
    bool_iter: base_types::BoolIterator,
    int_iter: base_types::IntIterator,
    float_iter: base_types::FloatIterator,
}

impl LaneTypeIterator {
    fn new() -> Self {
        Self {
            bool_iter: base_types::BoolIterator::new(),
            int_iter: base_types::IntIterator::new(),
            float_iter: base_types::FloatIterator::new(),
        }
    }
}

impl Iterator for LaneTypeIterator {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.bool_iter.next() {
            let next = LaneType {
                _bits: b as u64,
                _tag: LaneTypeTag::_BoolType(b),
            };
            Some(ValueType::Lane(next))
        } else if let Some(i) = self.int_iter.next() {
            let next = LaneType {
                _bits: i as u64,
                _tag: LaneTypeTag::_IntType(i),
            };
            Some(ValueType::Lane(next))
        } else if let Some(f) = self.float_iter.next() {
            let next = LaneType {
                _bits: f as u64,
                _tag: LaneTypeTag::_FloatType(f),
            };
            Some(ValueType::Lane(next))
        } else {
            None
        }
    }
}

/// A concrete SIMD vector type.
///
/// A vector type has a lane type which is an instance of `LaneType`,
/// and a positive number of lanes.
pub struct VectorType {
    _base: LaneType,
    _lanes: u8,
}

impl VectorType {
    /// Initialize a new integer type with `n` bits.
    pub fn _new(base: LaneType, lanes: u8) -> VectorType {
        VectorType {
            _base: base,
            _lanes: lanes,
        }
    }

    /// Return the number of lanes.
    pub fn _lane_count(&self) -> u8 {
        self._lanes
    }

    /// Return the number of bits in a lane.
    pub fn _lane_bits(&self) -> u64 {
        self._base._lane_bits()
    }
}

/// A flat bitvector type. Used for semantics description only.
pub struct _BVType;

impl _BVType {
    /// Initialize a new integer type with `n` bits.
    pub fn _new() -> Self {
        Self {}
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
    pub fn _rust_name(&self) -> String {
        format!("{}{}", _RUST_NAME_PREFIX, self.name().to_uppercase())
    }

    pub fn name(&self) -> &str {
        self.tag.name()
    }

    pub fn doc(&self) -> String {
        match self.tag {
            SpecialTypeTag::Flag(base_types::Flag::IFlags) => String::from("CPU flags representing the result of an integer comparison. These flags can be tested with an :type:`intcc` condition code.
            "),
            SpecialTypeTag::Flag(base_types::Flag::FFlags) => String::from("CPU flags representing the result of a floating point comparison. These flags can be tested with a :type:`floatcc` condition code.
            "),
        }
    }

    pub fn number(&self) -> u8 {
        self.tag.number()
    }
}

pub enum SpecialTypeTag {
    Flag(base_types::Flag),
}

impl SpecialTypeTag {
    pub fn name(&self) -> &str {
        match self {
            SpecialTypeTag::Flag(f) => f.name(),
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            SpecialTypeTag::Flag(f) => f.number(),
        }
    }
}

pub struct SpecialTypeIterator {
    flag_iter: base_types::FlagIterator,
}

impl SpecialTypeIterator {
    fn new() -> Self {
        Self {
            flag_iter: base_types::FlagIterator::new(),
        }
    }
}

impl Iterator for SpecialTypeIterator {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.flag_iter.next() {
            let next = SpecialType {
                tag: SpecialTypeTag::Flag(f),
            };
            Some(ValueType::Special(next))
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
