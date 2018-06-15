//! Cretonne ValueType hierarchy

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

// ValueType instances (i8, i32, ...) are provided in the `base.types` module.

/// A concrete SSA value type.
///
/// All SSA values have a type that is described by an instance of `ValueType`
/// or one of its subclasses.
trait ValueType {}

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
impl ValueType for _VectorType {}

/// A concrete scalar integer type.
struct _IntType {}
impl ValueType for _IntType {}
impl LaneType for _IntType {}

/// A concrete scalar floating point type.
struct _FloatType {}
impl ValueType for _FloatType {}
impl LaneType for _FloatType {}

/// A concrete scalar boolean type.
struct _BoolType {}
impl ValueType for _BoolType {}
impl LaneType for _BoolType {}

/// A type representing CPU flags.
///
/// Flags can't be stored in memory.
struct _FlagsType {}
impl ValueType for _FlagsType {}
impl SpecialType for _FlagsType {}

/// A flat bitvector type. Used for semantics description only.
struct _BVType {}
impl ValueType for _BVType {}
