//! Generate sources with type info.
//!
//! This generates a `types.rs` file which is included in
//! `lib/codegen/ir/types.rs`. The file provides constant definitions for the
//! most commonly used types, including all of the scalar types.
//!
//! This ensures that Python and Rust use the same type numbering.
//!
//! DEVELOPMENT NOTE: This will generate a `new_types.rs` file until complete.

use error;
use srcgen;

// FIXUP: Placeholder type.
type ValueType = ();

/// Emit a constant definition of a single value type.
fn _emit_type(ty: ValueType, fmt: &srcgen::Formatter) -> Result<(), error::Error> {
    unimplemented!();
    // TODO: In progress.
    // let name = ty.name.upper();
    // fmt.doc_comment(ty.__doc__);
    // fmt.line(
    //         "pub const {}: Type = Type({:#x});"
    //         .format(name, ty.number));
    // fmt.line();

    // Ok(())
}

/// Emit definition for all vector types with `bits` total size.
/// FIXUP: How many bits should this be?
fn emit_vectors(_bits: u16, _fmt: &srcgen::Formatter) -> Result<(), error::Error> {
    unimplemented!();
}

/// Emit types using the given formatter object.
fn emit_types(fmt: &srcgen::Formatter) -> Result<(), error::Error> {
    // DEVELOPMENT NOTE: This is the general scaffold, iterate through all
    // special types, and emit. Repeat for all lane types.
    // for spec in ValueType.all_special_types.iter() {
    //     emit_type(spec, fmt);
    // }
    // for ty in ValueType.all_lane_types.iter() {
    //     emit_type(ty, fmt);
    // }

    // Emit vector definitions for common SIMD sizes.
    emit_vectors(64, fmt)?;
    emit_vectors(128, fmt)?;
    emit_vectors(256, fmt)?;
    emit_vectors(512, fmt)?;

    Ok(())
}

/// Generate the `types.rs` file.
/// DEVELOPMENT NOTE: This will generate a `new_types.rs` file until complete.
pub fn generate(out_dir: &str) -> Result<(), error::Error> {
    // TODO: Create a formatter object, call `emit_types`, and update the file.
    let fmt = srcgen::Formatter::new();
    emit_types(&fmt)?;
    fmt.update_file("new_types.rs", Some(out_dir));
    Ok(())
}
