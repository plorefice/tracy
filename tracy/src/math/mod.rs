//! Mathematical operators and operations defined on them.
//!
//! This module contains the definitions of several mathematical and geometrical constructs that
//! are needed for the ray tracer, ie. coordinate systems, vectors, points, matrices etc.

mod coords;
mod matrix;

pub use coords::*;
pub use matrix::*;

/// Arbitrarily small number for floating point comparison.
pub const EPSILON: f32 = 1e-4;
