//! Generate sources with type info.
//!
//! This generates a `types.rs` file which is included in
//! `lib/codegen/ir/types.rs`. The file provides constant definitions for the
//! most commonly used types, including all of the scalar types.
//!
//! This ensures that Python and Rust use the same type numbering.
//!
//! DEVELOPMENT NOTE: This will generate a `new_types.rs` file until complete.

use cdsl::types as cdsl_types;
use error;
use srcgen;

/// Emit a constant definition of a single value type.
fn emit_type(ty: cdsl_types::ValueType, fmt: &mut srcgen::Formatter) -> Result<(), error::Error> {
    let name = ty.name().to_uppercase();
    fmt.doc_comment(&ty.doc());
    fmt.line(&format!(
        "pub const {}: Type = Type({:#x});\n",
        name,
        ty.number()
    ));

    Ok(())
}

/// Emit definition for all vector types with `bits` total size.
/// FIXUP: What type should `_bits` be defined as?
fn _emit_vectors(bits: u16, _fmt: &srcgen::Formatter) -> Result<(), error::Error> {
    let _size: u16 = bits / 8;
    unimplemented!();
}

/// Emit types using the given formatter object.
fn emit_types(fmt: &mut srcgen::Formatter) -> Result<(), error::Error> {
    // Emit all of the special types, such as types for CPU flags.
    for spec in cdsl_types::ValueType::all_special_types() {
        emit_type(spec, fmt)?;
    }

    // Emit all of the lane types, such integers, floats, and booleans.
    for ty in cdsl_types::ValueType::all_lane_types() {
        emit_type(ty, fmt)?;
    }

    // Emit vector definitions for common SIMD sizes.
    // emit_vectors(64, fmt)?;
    // emit_vectors(128, fmt)?;
    // emit_vectors(256, fmt)?;
    // emit_vectors(512, fmt)?;

    Ok(())
}

/// Generate the types file.
pub fn generate(filename: &str, out_dir: &str) -> Result<(), error::Error> {
    let mut fmt = srcgen::Formatter::new();
    emit_types(&mut fmt)?;
    fmt.update_file(filename, out_dir)?;
    Ok(())
}
