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

/// Emit definition for all vector types with `bits` total size.
fn _emit_vectors(fmt: srcgen::Formatter) -> Result<(), error::Error> {
    unimplemented!();
}

/// Emit types using the given formatter object.
fn emit_types(fmt: &srcgen::Formatter) -> Result<(), error::Error> {
    unimplemented!();
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
