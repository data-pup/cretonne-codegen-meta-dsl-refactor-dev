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

// class ValueType(object):
// class SpecialType(ValueType):
// class LaneType(ValueType):
// class VectorType(ValueType):
// class IntType(LaneType):
// class FloatType(LaneType):
// class BoolType(LaneType):
// class FlagsType(SpecialType):
// class BVType(ValueType):

trait ValueType {}
trait LaneType {}
trait SpecialType {}

struct _VectorType {}
impl ValueType for _VectorType {}

struct _IntType {}
impl LaneType for _IntType {}

struct _FloatType {}
impl LaneType for _FloatType {}

struct _BoolType {}
impl LaneType for _BoolType {}

struct _FlagsType {}
impl SpecialType for _FlagsType {}

struct _BVType {}
impl ValueType for _BVType {}
