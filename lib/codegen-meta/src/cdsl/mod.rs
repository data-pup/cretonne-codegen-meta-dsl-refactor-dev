//! Cretonne DSL classes.
//!
//! This module defines the classes that are used to define Cretonne
//! instructions and other entitties.

pub mod types;

/// Convert the string `s` to CamelCase.
fn camel_case(s: &str) -> String {
    unimplemented!();
}

/// Check if `x` is a power of two.
/// TODO: This might need to be generic.
fn is_power_of_two(x: u8) -> bool {
    x > 0 && x & (x - 1) == 0
}

/// Compute the next power of two that is greater than `x`.
fn next_power_of_two(x: u8) -> u8 {
    let mut s = 1;
    let mut res = x;
    while res & (res + 1) != 0 {
        res |= res >> s;
        s *= 2;
    }

    res + 1
}
