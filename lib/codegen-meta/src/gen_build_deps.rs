//! Generate build dependencies for Cargo.
//!
//! The `build.rs` script is invoked by cargo when building lib/codegen to
//! generate Rust code from the instruction descriptions. Cargo needs to know when
//! it is necessary to rerun the build script.
//!
//! If the build script outputs lines of the form:
//!     cargo:rerun-if-changed=/path/to/file
//!
//! cargo will rerun the build script when those files have changed since the last
//! build.

use error;

/// Recursively find all interesting source files and directories in the
/// directory tree starting at top. Yield a path to each file.
fn source_files() -> Result<Vec<String>, error::Error> {
    unimplemented!();
}

pub fn generate() -> Result<(), error::Error> {
    println!("Dependencies from Rust meta language directory:");
    // let meta = // FIXUP: Get codegen-meta crate directory.
    source_files()?
        .into_iter()
        .for_each(|p| println!("cargo:rerun-if-changed={}", p));

    Ok(())
}
