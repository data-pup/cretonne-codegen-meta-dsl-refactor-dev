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

use std::path;

/// Recursively find all interesting source files and directories in the
/// directory tree starting at top. Yield a path to each file.
fn source_files() -> Result<Vec<String>, error::Error> {
    unimplemented!();
}

pub fn generate(meta_dir: &path::PathBuf) -> Result<(), error::Error> {
    println!("Dependencies from Rust meta language directory:");

    // FIXUP: Get codegen-meta crate directory.
    // The original python code uses this line of code:
    // meta = dirname(abspath(__file__))
    // NOTE: Read into what `__file__` is doing here.
    // dirname, of the absolute path to this file?
    // let meta = TODO ...

    source_files()?
        .into_iter()
        .for_each(|p| println!("cargo:rerun-if-changed={}", p));

    Ok(())
}
