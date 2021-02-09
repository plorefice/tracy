//! Mathematical operators and operations defined on them.
//!
//! This module contains the definitions of several mathematical and geometrical constructs that
//! are needed for the ray tracer, ie. coordinate systems, vectors, points, matrices etc.

mod coords;
mod matrix;
mod ray;

pub use coords::*;
pub use matrix::*;
pub use ray::*;
